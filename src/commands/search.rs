use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct SearchResults {
    query: String,
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

impl Tableable for SearchResults {
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

pub async fn execute(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    query: &str,
    mode: &str,
    count: usize,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let tweets = api.search_tweets(query, mode, count).await?;
    let display = SearchResults {
        query: query.to_string(),
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
    };
    output::render(format, &display, None);
    Ok(())
}
