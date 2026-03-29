use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::intel::tracker::PostTracker;
use crate::output::{self, OutputFormat};
use crate::providers::xapi::XApi;
use std::sync::Arc;

/// Snapshot all recent posts (designed for cron). Default: last 48 hours.
/// Also checks pending replies for reply-backs.
pub async fn track_run(
    ctx: Arc<AppContext>,
    format: OutputFormat,
) -> Result<(), XmasterError> {
    let tracker = PostTracker::open()?;
    let summary = tracker.snapshot_all_recent(&ctx, 48).await?;

    // Check pending replies for reply-backs (silent, never fails the run)
    let reply_backs_checked = check_reply_backs(&ctx).await;

    let mut meta = serde_json::json!({});
    if reply_backs_checked > 0 {
        meta["reply_backs_checked"] = reply_backs_checked.into();
    }

    output::render(format, &summary, Some(meta));
    Ok(())
}

/// Check if targets replied back to our replies.
async fn check_reply_backs(ctx: &Arc<AppContext>) -> u32 {
    let store = match crate::intel::store::IntelStore::open() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let pending = match store.get_pending_replies(72) {
        Ok(p) => p,
        Err(_) => return 0,
    };

    if pending.is_empty() {
        return 0;
    }

    let api = XApi::new(ctx.clone());
    let mut checked = 0u32;

    for pr in &pending {
        // Fetch replies to our reply tweet
        match api.get_replies(&pr.reply_tweet_id, 10).await {
            Ok(replies) => {
                let target_user = pr.target_username.as_deref().unwrap_or("");
                let got_reply = replies.iter().any(|r| {
                    r.author_username.as_deref()
                        .map(|u| u.to_lowercase() == target_user.to_lowercase())
                        .unwrap_or(false)
                });
                let _ = store.set_reply_back(pr.id, got_reply);
                checked += 1;
            }
            Err(_) => {
                // Timeout old pending replies (>72h without check = assume no reply)
                let age_hours = (chrono::Utc::now().timestamp() - pr.performed_at) / 3600;
                if age_hours > 72 {
                    let _ = store.set_reply_back(pr.id, false);
                    checked += 1;
                }
            }
        }
    }

    checked
}

/// Show which posts are being tracked and their latest snapshot age.
pub async fn track_status(
    _ctx: Arc<AppContext>,
    format: OutputFormat,
) -> Result<(), XmasterError> {
    let tracker = PostTracker::open()?;
    let status = tracker.tracking_status()?;

    if status.total == 0 {
        return Err(XmasterError::NotFound(
            "No posts are being tracked yet. Post something first with `xmaster post`, then run `xmaster track run`".into(),
        ));
    }

    output::render(format, &status, None);
    Ok(())
}

/// Snapshot follower count and detect new/lost followers.
pub async fn track_followers(
    ctx: Arc<AppContext>,
    format: OutputFormat,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let user_id = api.get_authenticated_user_id().await?;

    // Get current account stats
    let me = api.get_user_by_id(&user_id).await?;
    let metrics = me.public_metrics.as_ref();
    let followers_count = metrics.map(|m| m.followers_count as i64).unwrap_or(0);
    let following_count = metrics.map(|m| m.following_count as i64).unwrap_or(0);
    let tweet_count = metrics.map(|m| m.tweet_count as i64).unwrap_or(0);

    let tracker = PostTracker::open()?;

    // Snapshot account stats
    let snapshot = tracker.snapshot_account(followers_count, following_count, tweet_count)?;

    // Get full follower list for diffing
    let follower_data = api.get_user_followers(&user_id, 1000).await?;
    let follower_tuples: Vec<(String, String, i64)> = follower_data.iter().map(|u| {
        (
            u.id.clone(),
            u.username.clone(),
            u.public_metrics.as_ref().map(|m| m.followers_count as i64).unwrap_or(0),
        )
    }).collect();

    // Diff against previous
    let changes = tracker.diff_followers(&follower_tuples)?;

    // Store current list
    tracker.store_follower_list(&follower_tuples)?;

    // Combine output
    let output_data = FollowerTrackResult {
        account: snapshot,
        changes,
    };
    output::render(format, &output_data, None);
    Ok(())
}

/// Show follower growth history.
pub async fn follower_growth(
    _ctx: Arc<AppContext>,
    format: OutputFormat,
    days: i64,
) -> Result<(), XmasterError> {
    let tracker = PostTracker::open()?;
    let history = tracker.follower_history(days)?;

    if history.is_empty() {
        return Err(XmasterError::NotFound(
            "No follower history yet. Run `xmaster track followers` first.".into(),
        ));
    }

    let output_data = FollowerGrowthResult { days, snapshots: history };
    output::render(format, &output_data, None);
    Ok(())
}

use serde::Serialize;
use crate::intel::tracker::{AccountSnapshot, FollowerChange};

#[derive(Serialize)]
struct FollowerTrackResult {
    account: AccountSnapshot,
    changes: FollowerChange,
}

impl crate::output::Tableable for FollowerTrackResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Metric", "Value"]);
        table.add_row(vec!["Followers", &self.account.followers.to_string()]);
        table.add_row(vec!["Following", &self.account.following.to_string()]);
        table.add_row(vec!["Tweets", &self.account.tweets.to_string()]);
        let change_str = if self.account.followers_change >= 0 {
            format!("+{}", self.account.followers_change)
        } else {
            self.account.followers_change.to_string()
        };
        table.add_row(vec!["Followers Change", &change_str]);
        if !self.changes.new_followers.is_empty() {
            let names: Vec<String> = self.changes.new_followers.iter()
                .map(|f| format!("@{}", f.username))
                .collect();
            table.add_row(vec!["New Followers", &names.join(", ")]);
        }
        if !self.changes.lost_followers.is_empty() {
            let names: Vec<String> = self.changes.lost_followers.iter()
                .map(|f| format!("@{}", f.username))
                .collect();
            table.add_row(vec!["Lost Followers", &names.join(", ")]);
        }
        table
    }
}

#[derive(Serialize)]
struct FollowerGrowthResult {
    days: i64,
    snapshots: Vec<AccountSnapshot>,
}

impl crate::output::Tableable for FollowerGrowthResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Date", "Followers", "Following", "Tweets"]);
        for s in &self.snapshots {
            table.add_row(vec![
                s.snapshot_at.chars().take(10).collect::<String>(),
                s.followers.to_string(),
                s.following.to_string(),
                s.tweets.to_string(),
            ]);
        }
        table
    }
}
