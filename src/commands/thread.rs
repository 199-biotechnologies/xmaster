use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, CsvRenderable, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct ThreadResult {
    tweet_ids: Vec<String>,
    total: usize,
    succeeded: usize,
    failed: usize,
}

impl Tableable for ThreadResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["#", "Tweet ID", "Status"]);
        for (i, id) in self.tweet_ids.iter().enumerate() {
            table.add_row(vec![
                (i + 1).to_string(),
                id.clone(),
                "Posted".to_string(),
            ]);
        }
        if self.failed > 0 {
            table.add_row(vec![
                "".to_string(),
                format!("{} tweet(s) failed", self.failed),
                "Failed".to_string(),
            ]);
        }
        table
    }
}

impl CsvRenderable for ThreadResult {
    fn csv_headers() -> Vec<&'static str> {
        vec!["index", "tweet_id", "status"]
    }
    fn csv_rows(&self) -> Vec<Vec<String>> {
        self.tweet_ids
            .iter()
            .enumerate()
            .map(|(i, id)| vec![(i + 1).to_string(), id.clone(), "posted".into()])
            .collect()
    }
}

pub async fn execute(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    texts: &[String],
    media: &[String],
) -> Result<(), XmasterError> {
    if texts.is_empty() {
        return Err(XmasterError::Api {
            provider: "x",
            code: "invalid_input",
            message: "Thread must contain at least one tweet".into(),
        });
    }

    let api = XApi::new(ctx.clone());

    // Upload media if provided (attach to first tweet only)
    let media_ids = if !media.is_empty() {
        let mut ids = Vec::new();
        for path in media {
            let id = api.upload_media(path).await?;
            ids.push(id);
        }
        Some(ids)
    } else {
        None
    };

    let mut posted_ids: Vec<String> = Vec::new();
    let mut failed = 0usize;

    for (i, text) in texts.iter().enumerate() {
        let reply_to = if i == 0 {
            None
        } else {
            posted_ids.last().map(|s| s.as_str())
        };
        let tweet_media = if i == 0 { media_ids.as_deref() } else { None };

        match api
            .create_tweet(text, reply_to, None, tweet_media, None, None)
            .await
        {
            Ok(resp) => posted_ids.push(resp.id),
            Err(e) => {
                failed += 1;
                let remaining = texts.len() - i - 1;
                failed += remaining;
                eprintln!(
                    "Thread broken at tweet {}/{}: {e}. {} tweet(s) not posted.",
                    i + 1,
                    texts.len(),
                    remaining
                );
                break;
            }
        }
    }

    let display = ThreadResult {
        total: texts.len(),
        succeeded: posted_ids.len(),
        failed,
        tweet_ids: posted_ids,
    };
    output::render(format, &display, None);
    Ok(())
}
