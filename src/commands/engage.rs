use crate::cli::parse_tweet_id;
use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct ActionResult {
    action: String,
    tweet_id: String,
    success: bool,
}

impl Tableable for ActionResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Action", "Tweet ID", "Status"]);
        table.add_row(vec![
            self.action.as_str(),
            self.tweet_id.as_str(),
            if self.success { "OK" } else { "Failed" },
        ]);
        table
    }
}

fn render_success(format: OutputFormat, action_name: &str, tweet_id: String) {
    let display = ActionResult {
        action: action_name.to_string(),
        tweet_id,
        success: true,
    };
    output::render(format, &display, None);
}

pub async fn delete(ctx: Arc<AppContext>, format: OutputFormat, id: &str) -> Result<(), XmasterError> {
    let api = XApi::new(ctx);
    let tweet_id = parse_tweet_id(id);
    api.delete_tweet(&tweet_id).await?;
    render_success(format, "delete", tweet_id);
    Ok(())
}

pub async fn like(ctx: Arc<AppContext>, format: OutputFormat, id: &str) -> Result<(), XmasterError> {
    let api = XApi::new(ctx);
    let tweet_id = parse_tweet_id(id);
    api.like_tweet(&tweet_id).await?;
    render_success(format, "like", tweet_id);
    Ok(())
}

pub async fn unlike(ctx: Arc<AppContext>, format: OutputFormat, id: &str) -> Result<(), XmasterError> {
    let api = XApi::new(ctx);
    let tweet_id = parse_tweet_id(id);
    api.unlike_tweet(&tweet_id).await?;
    render_success(format, "unlike", tweet_id);
    Ok(())
}

pub async fn retweet(ctx: Arc<AppContext>, format: OutputFormat, id: &str) -> Result<(), XmasterError> {
    let api = XApi::new(ctx);
    let tweet_id = parse_tweet_id(id);
    api.retweet(&tweet_id).await?;
    render_success(format, "retweet", tweet_id);
    Ok(())
}

pub async fn unretweet(ctx: Arc<AppContext>, format: OutputFormat, id: &str) -> Result<(), XmasterError> {
    let api = XApi::new(ctx);
    let tweet_id = parse_tweet_id(id);
    api.unretweet(&tweet_id).await?;
    render_success(format, "unretweet", tweet_id);
    Ok(())
}

pub async fn bookmark(ctx: Arc<AppContext>, format: OutputFormat, id: &str) -> Result<(), XmasterError> {
    let api = XApi::new(ctx);
    let tweet_id = parse_tweet_id(id);
    api.bookmark_tweet(&tweet_id).await?;
    render_success(format, "bookmark", tweet_id);
    Ok(())
}

pub async fn unbookmark(ctx: Arc<AppContext>, format: OutputFormat, id: &str) -> Result<(), XmasterError> {
    let api = XApi::new(ctx);
    let tweet_id = parse_tweet_id(id);
    api.unbookmark_tweet(&tweet_id).await?;
    render_success(format, "unbookmark", tweet_id);
    Ok(())
}
