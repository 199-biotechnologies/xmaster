use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::intel::store::IntelStore;
use crate::output::{self, OutputFormat, Tableable};
use crate::providers::xai::XaiSearch;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct RecommendCandidate {
    pub rank: usize,
    pub username: String,
    pub followers: u64,
    pub reply_rate: f64,
    pub score: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecommendResult {
    pub candidates: Vec<RecommendCandidate>,
    pub suggested_next_commands: Vec<String>,
}

impl Tableable for RecommendResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Rank", "@Username", "Followers", "Reply Rate", "Score", "Source"]);
        for c in &self.candidates {
            table.add_row(vec![
                c.rank.to_string(),
                format!("@{}", c.username),
                format_followers(c.followers),
                if c.reply_rate > 0.0 {
                    format!("{:.0}%", c.reply_rate * 100.0)
                } else {
                    "—".into()
                },
                format!("{:.2}", c.score),
                c.source.clone(),
            ]);
        }
        table
    }
}

fn format_followers(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

// ---------------------------------------------------------------------------
// Candidate collection (internal)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct RawCandidate {
    username: String,
    followers: u64,
    reply_rate: f64,
    source: String,
    relevance: f64,
}

// ---------------------------------------------------------------------------
// Command handler
// ---------------------------------------------------------------------------

pub async fn recommend(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    topic: Option<&str>,
    min_followers: u32,
    count: usize,
) -> Result<(), XmasterError> {
    let mut candidates: HashMap<String, RawCandidate> = HashMap::new();

    // Phase 1a: Local history — proven reciprocators
    if let Ok(store) = IntelStore::open() {
        if let Ok(reciprocators) = store.get_top_reciprocators(min_followers as i64, 20) {
            for r in reciprocators {
                let username = r.username.to_lowercase();
                candidates.entry(username.clone()).or_insert(RawCandidate {
                    username: r.username,
                    followers: r.avg_followers as u64,
                    reply_rate: r.reply_rate,
                    source: "history".into(),
                    relevance: 0.3,
                });
            }
        }
    }

    // Phase 1b: Live mentions — people already talking to you
    let xapi = crate::providers::xapi::XApi::new(ctx.clone());
    if let Ok(user_id) = xapi.get_authenticated_user_id().await {
        if let Ok(mentions) = xapi.get_user_mentions(&user_id, 20).await {
            for tweet in &mentions {
                if let Some(username) = &tweet.author_username {
                    let key = username.to_lowercase();
                    if candidates.contains_key(&key) {
                        continue;
                    }
                    let followers = tweet.author_followers.unwrap_or(0);
                    candidates.entry(key).or_insert(RawCandidate {
                        username: username.clone(),
                        followers,
                        reply_rate: 0.0,
                        source: "mentions".into(),
                        relevance: 0.7,
                    });
                }
            }
        }
    }

    // Phase 1c: Topic discovery via xAI search
    if let Some(topic_str) = topic {
        let xai = XaiSearch::new(ctx.clone());
        if let Ok(result) = xai.search_posts(topic_str, 20, None, None, None).await {
            // Extract usernames from citations and text
            let usernames = extract_usernames_from_text(&result.text);
            for username in usernames {
                let key = username.to_lowercase();
                if candidates.contains_key(&key) {
                    continue;
                }
                candidates.entry(key).or_insert(RawCandidate {
                    username,
                    followers: 0,
                    reply_rate: 0.0,
                    source: "topic".into(),
                    relevance: 1.0,
                });
            }
        }
    }

    // Phase 1d: Enrich with reciprocity data from store
    if let Ok(store) = IntelStore::open() {
        for (_, cand) in candidates.iter_mut() {
            if cand.reply_rate == 0.0 {
                if let Ok(Some(info)) = store.get_engagement_reciprocity(&cand.username) {
                    cand.reply_rate = info.reply_rate;
                }
            }
        }
    }

    // Filter by min_followers (skip candidates with 0 followers unless from topic/mentions)
    let filtered: Vec<RawCandidate> = candidates
        .into_values()
        .filter(|c| c.followers >= min_followers as u64 || c.source != "history")
        .collect();

    if filtered.is_empty() {
        return Err(XmasterError::NotFound(
            "No recommendation candidates found. Try: `xmaster engage recommend --topic \"your niche\"` or engage with more accounts first".into(),
        ));
    }

    // Phase 2: Score
    let mut scored: Vec<RecommendCandidate> = filtered
        .into_iter()
        .map(|c| {
            let reciprocity = c.reply_rate; // 0-1
            let reach = if c.followers > 0 {
                ((c.followers as f64).log2() / 20.0).min(1.0)
            } else {
                0.0
            };
            let freshness = 1.0; // all candidates are from live/recent data
            let relevance = c.relevance;

            let score = 0.4 * reciprocity + 0.3 * reach + 0.2 * freshness + 0.1 * relevance;

            RecommendCandidate {
                rank: 0, // assigned after sort
                username: c.username,
                followers: c.followers,
                reply_rate: c.reply_rate,
                score,
                source: c.source,
            }
        })
        .collect();

    // Phase 3: Rank
    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(count);
    for (i, c) in scored.iter_mut().enumerate() {
        c.rank = i + 1;
    }

    let suggested_next_commands: Vec<String> = scored
        .iter()
        .map(|c| format!("xmaster search \"from:{}\" -c 5", c.username))
        .collect();

    let result = RecommendResult {
        candidates: scored,
        suggested_next_commands,
    };

    let metadata = serde_json::json!({
        "suggested_next_commands": result.suggested_next_commands,
    });

    output::render(format, &result, Some(metadata));
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Watchlist CRUD
// ---------------------------------------------------------------------------

pub async fn watchlist_add(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    username: &str,
    topic: Option<&str>,
) -> Result<(), XmasterError> {
    let store = IntelStore::open().map_err(|e| XmasterError::Config(format!("DB error: {e}")))?;
    let api = crate::providers::xapi::XApi::new(ctx.clone());

    // Fetch user info to get ID and follower count
    let user = api.get_user_by_username(username).await?;
    let followers = user.public_metrics.as_ref().map(|m| m.followers_count as i64).unwrap_or(0);

    store.add_watchlist(username, Some(&user.id), topic, followers)
        .map_err(|e| XmasterError::Config(format!("DB error: {e}")))?;

    #[derive(Serialize)]
    struct WatchlistAddResult { username: String, user_id: String, followers: i64, topic: Option<String>, status: String }
    impl Tableable for WatchlistAddResult {
        fn to_table(&self) -> comfy_table::Table {
            let mut t = comfy_table::Table::new();
            t.set_header(vec!["Field", "Value"]);
            t.add_row(vec!["Username", &format!("@{}", self.username)]);
            t.add_row(vec!["Followers", &format_followers(self.followers as u64)]);
            t.add_row(vec!["Status", &self.status]);
            t
        }
    }
    let display = WatchlistAddResult {
        username: username.to_string(), user_id: user.id, followers, topic: topic.map(String::from), status: "added".into(),
    };
    output::render(format, &display, None);
    Ok(())
}

pub async fn watchlist_list(format: OutputFormat) -> Result<(), XmasterError> {
    let store = IntelStore::open().map_err(|e| XmasterError::Config(format!("DB error: {e}")))?;
    let entries = store.list_watchlist().map_err(|e| XmasterError::Config(format!("DB error: {e}")))?;

    if entries.is_empty() {
        return Err(XmasterError::NotFound("Watchlist is empty. Add accounts with: xmaster engage watchlist add <username>".into()));
    }

    #[derive(Serialize)]
    struct WatchlistDisplay { accounts: Vec<crate::intel::store::WatchlistEntry> }
    impl Tableable for WatchlistDisplay {
        fn to_table(&self) -> comfy_table::Table {
            let mut t = comfy_table::Table::new();
            t.set_header(vec!["Username", "Followers", "Topic"]);
            for a in &self.accounts {
                t.add_row(vec![
                    format!("@{}", a.username),
                    format_followers(a.followers as u64),
                    a.topic.clone().unwrap_or_default(),
                ]);
            }
            t
        }
    }

    output::render(format, &WatchlistDisplay { accounts: entries }, None);
    Ok(())
}

pub async fn watchlist_remove(format: OutputFormat, username: &str) -> Result<(), XmasterError> {
    let store = IntelStore::open().map_err(|e| XmasterError::Config(format!("DB error: {e}")))?;
    let removed = store.remove_watchlist(username).map_err(|e| XmasterError::Config(format!("DB error: {e}")))?;

    if !removed {
        return Err(XmasterError::NotFound(format!("@{username} not in watchlist")));
    }

    #[derive(Serialize)]
    struct RemoveResult { username: String, status: String }
    impl Tableable for RemoveResult {
        fn to_table(&self) -> comfy_table::Table {
            let mut t = comfy_table::Table::new();
            t.add_row(vec![&format!("@{} removed from watchlist", self.username)]);
            t
        }
    }
    output::render(format, &RemoveResult { username: username.to_string(), status: "removed".into() }, None);
    Ok(())
}

// ---------------------------------------------------------------------------
// engage feed — find fresh posts from big accounts to reply to NOW
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct FeedPost {
    pub id: String,
    pub author: String,
    pub author_followers: u64,
    pub text: String,
    pub age_minutes: i64,
    pub likes: u64,
    pub replies: u64,
    pub reply_command: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeedResult {
    pub topic: String,
    pub posts: Vec<FeedPost>,
    pub total_found: usize,
    pub filtered_by_followers: usize,
}

impl Tableable for FeedResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Age", "Author", "Followers", "Text", "Likes", "Reply cmd"]);
        for p in &self.posts {
            let text_preview: String = p.text.chars().take(60).collect::<String>()
                + if p.text.chars().count() > 60 { "..." } else { "" };
            table.add_row(vec![
                format!("{}m", p.age_minutes),
                format!("@{}", p.author),
                format_followers(p.author_followers),
                text_preview,
                p.likes.to_string(),
                p.reply_command.clone(),
            ]);
        }
        table
    }
}

pub async fn feed(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    topic: &str,
    min_followers: u64,
    max_age_mins: u64,
    count: usize,
) -> Result<(), XmasterError> {
    let api = crate::providers::xapi::XApi::new(ctx.clone());

    // Phase 1: Check watchlist accounts first (saves API search calls)
    let mut watchlist_tweets = Vec::new();
    if let Ok(store) = IntelStore::open() {
        if let Ok(watchlist) = store.list_watchlist() {
            for entry in &watchlist {
                if let Some(ref uid) = entry.user_id {
                    let start_time = {
                        let since = chrono::Utc::now() - chrono::Duration::minutes(max_age_mins as i64);
                        since.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                    };
                    if let Ok(tweets) = api.get_user_tweets_paginated(uid, 5, Some(&start_time), None).await {
                        for mut t in tweets {
                            // Inject known follower count from watchlist (avoids missing data)
                            if t.author_followers.is_none() {
                                t.author_followers = Some(entry.followers as u64);
                            }
                            if t.author_username.is_none() {
                                t.author_username = Some(entry.username.clone());
                            }
                            watchlist_tweets.push(t);
                        }
                    }
                }
            }
        }
    }

    // Phase 2: Cold search for discovery (only if watchlist didn't fill count)
    let start_time = {
        let now = chrono::Utc::now();
        let since = now - chrono::Duration::minutes(max_age_mins as i64);
        since.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    };

    let search_tweets = if watchlist_tweets.len() < count {
        api.search_tweets_paginated(
            topic,
            "recent",
            100.min(count * 5),
            Some(&start_time),
            None,
        ).await.unwrap_or_default()
    } else {
        Vec::new()
    };

    // Combine: watchlist first, then search results
    let mut seen_ids = std::collections::HashSet::new();
    let mut tweets = Vec::new();
    for t in watchlist_tweets.into_iter().chain(search_tweets.into_iter()) {
        if seen_ids.insert(t.id.clone()) {
            tweets.push(t);
        }
    }

    let now = chrono::Utc::now();
    let mut posts: Vec<FeedPost> = Vec::new();
    let total_found = tweets.len();
    let mut filtered_count = 0usize;

    for t in tweets {
        let author_followers = t.author_followers.unwrap_or(0);
        if author_followers < min_followers {
            filtered_count += 1;
            continue;
        }

        // Skip replies and retweets — we want original posts
        if let Some(refs) = &t.referenced_tweets {
            if refs.iter().any(|r| r.ref_type == "retweeted" || r.ref_type == "replied_to") {
                continue;
            }
        }

        let age_minutes = t.created_at.as_deref()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| (now - dt.with_timezone(&chrono::Utc)).num_minutes())
            .unwrap_or(0);

        let metrics = t.public_metrics.as_ref();
        let author = t.author_username
            .unwrap_or_else(|| t.author_id.unwrap_or_default());

        posts.push(FeedPost {
            reply_command: format!("xmaster reply {} \"your reply\"", t.id),
            id: t.id,
            author: author.clone(),
            author_followers,
            text: t.text,
            age_minutes,
            likes: metrics.map(|m| m.like_count).unwrap_or(0),
            replies: metrics.map(|m| m.reply_count).unwrap_or(0),
        });
    }

    // Sort by freshest first (lowest age)
    posts.sort_by_key(|p| p.age_minutes);
    posts.truncate(count);

    // Auto-add high-value accounts from search to watchlist (silent, never fails)
    if let Ok(store) = IntelStore::open() {
        for p in &posts {
            if p.author_followers >= 10_000 {
                let _ = store.add_watchlist(&p.author, None, Some(topic), p.author_followers as i64);
            }
        }
    }

    let result = FeedResult {
        topic: topic.to_string(),
        posts,
        total_found,
        filtered_by_followers: filtered_count,
    };

    output::render(format, &result, None);
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract @usernames from xAI search result text.
fn extract_usernames_from_text(text: &str) -> Vec<String> {
    let mut usernames = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for word in text.split_whitespace() {
        let trimmed = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '_');
        if let Some(name) = trimmed.strip_prefix('@') {
            let clean: String = name
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if clean.len() >= 2 && seen.insert(clean.to_lowercase()) {
                usernames.push(clean);
            }
        }
    }

    usernames
}

