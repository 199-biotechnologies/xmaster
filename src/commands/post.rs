use crate::cli::parse_tweet_id;
use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct PostResult {
    id: String,
    text: String,
}

impl Tableable for PostResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Field", "Value"]);
        table.add_row(vec!["Tweet ID", &self.id]);
        table.add_row(vec!["Text", &self.text]);
        table
    }
}

pub async fn execute(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    text: &str,
    reply_to: Option<&str>,
    quote: Option<&str>,
    media: &[String],
    poll: Option<&str>,
    poll_duration: u64,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());

    // Parse reply_to ID from URL or raw ID
    let reply_id = reply_to.map(parse_tweet_id);
    let quote_id = quote.map(parse_tweet_id);

    // Upload media files if provided
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

    // Parse poll options
    let poll_options: Option<Vec<String>> = poll.map(|p| {
        p.split(',').map(|s| s.trim().to_string()).collect()
    });

    let result = api
        .create_tweet(
            text,
            reply_id.as_deref(),
            quote_id.as_deref(),
            media_ids.as_deref(),
            poll_options.as_deref(),
            Some(poll_duration),
        )
        .await
        .map_err(|err| {
            // Add contextual hint when post fails with 403
            if let XmasterError::AuthMissing { provider, ref message } = err {
                if message.contains("403") {
                    return XmasterError::Api {
                        provider,
                        code: "forbidden",
                        message: format!(
                            "{message}. Hint: Check your app permissions — ensure Read+Write is enabled"
                        ),
                    };
                }
            }
            err
        })?;

    let tweet_id = result.id.clone();
    let display = PostResult {
        id: result.id,
        text: result.text,
    };
    output::render(format, &display, None);

    // Undo hint (only in table mode so it doesn't pollute JSON/CSV stdout)
    if format == OutputFormat::Table {
        eprintln!("Delete: xmaster delete {tweet_id}");
    }
    Ok(())
}
