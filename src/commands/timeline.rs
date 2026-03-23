use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, CsvRenderable, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct TweetList {
    tweets: Vec<TweetRow>,
}

#[derive(Serialize)]
struct TweetRow {
    id: String,
    author: String,
    text: String,
    likes: u64,
    retweets: u64,
    date: String,
}

impl Tableable for TweetList {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["ID", "Author", "Text", "Likes", "RTs", "Date"]);
        for t in &self.tweets {
            let truncated = if t.text.len() > 80 {
                format!("{}...", &t.text[..77])
            } else {
                t.text.clone()
            };
            table.add_row(vec![
                t.id.clone(),
                t.author.clone(),
                truncated,
                t.likes.to_string(),
                t.retweets.to_string(),
                t.date.clone(),
            ]);
        }
        table
    }
}

impl CsvRenderable for TweetList {
    fn csv_headers() -> Vec<&'static str> {
        vec!["id", "author", "text", "likes", "retweets", "date"]
    }

    fn csv_rows(&self) -> Vec<Vec<String>> {
        self.tweets
            .iter()
            .map(|t| {
                vec![
                    t.id.clone(),
                    t.author.clone(),
                    t.text.clone(),
                    t.likes.to_string(),
                    t.retweets.to_string(),
                    t.date.clone(),
                ]
            })
            .collect()
    }
}

fn tweets_to_list(tweets: Vec<crate::providers::xapi::TweetData>) -> TweetList {
    TweetList {
        tweets: tweets.into_iter().map(|t| {
            let metrics = t.public_metrics.as_ref();
            TweetRow {
                id: t.id,
                author: t.author_username
                    .map(|u| format!("@{u}"))
                    .unwrap_or_else(|| t.author_id.unwrap_or_default()),
                text: t.text,
                likes: metrics.map(|m| m.like_count).unwrap_or(0),
                retweets: metrics.map(|m| m.retweet_count).unwrap_or(0),
                date: t.created_at.unwrap_or_default(),
            }
        }).collect(),
    }
}

pub async fn timeline(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    user: Option<&str>,
    count: usize,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let user_id = match user {
        Some(username) => {
            let u = api.get_user_by_username(username).await?;
            u.id
        }
        None => api.get_authenticated_user_id().await?,
    };
    // TODO: Pagination — when xapi exposes next_token from get_user_tweets(),
    // loop here while next_token.is_some() && pages < max_pages, accumulating
    // tweets across pages. The --all / --max-pages flags will be wired in cli.rs.
    let tweets = api.get_user_tweets(&user_id, count).await?;
    output::render_csv(format, &tweets_to_list(tweets), None);
    Ok(())
}

pub async fn mentions(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    count: usize,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let user_id = api.get_authenticated_user_id().await?;
    // TODO: Pagination — same pattern as timeline(): loop with next_token
    // from get_user_mentions() when --all is passed.
    let tweets = api.get_user_mentions(&user_id, count).await?;
    output::render_csv(format, &tweets_to_list(tweets), None);
    Ok(())
}

pub async fn bookmarks(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    count: usize,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    // TODO: Pagination — same pattern as timeline(): loop with next_token
    // from get_bookmarks() when --all is passed.
    let tweets = api.get_bookmarks(count).await?;
    output::render_csv(format, &tweets_to_list(tweets), None);
    Ok(())
}
