use crate::cli::parse_tweet_id;
use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, CsvRenderable, OutputFormat, Tableable};
use reqwest_oauth1::OAuthClientProvider;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct ApiEnvelope {
    data: Option<TweetMetricsData>,
}

#[derive(Debug, Deserialize)]
struct TweetMetricsData {
    id: String,
    #[serde(default)]
    public_metrics: Option<PublicMetrics>,
    #[serde(default)]
    non_public_metrics: Option<NonPublicMetrics>,
}

#[derive(Debug, Deserialize, Default)]
struct PublicMetrics {
    #[serde(default)]
    like_count: u64,
    #[serde(default)]
    retweet_count: u64,
    #[serde(default)]
    reply_count: u64,
    #[serde(default)]
    impression_count: u64,
    #[serde(default)]
    quote_count: u64,
    #[serde(default)]
    bookmark_count: u64,
}

#[derive(Debug, Deserialize, Default)]
struct NonPublicMetrics {
    #[serde(default)]
    url_link_clicks: u64,
    #[serde(default)]
    user_profile_clicks: u64,
}

#[derive(Serialize)]
struct MetricsDisplay {
    tweet_id: String,
    impressions: u64,
    likes: u64,
    retweets: u64,
    replies: u64,
    quotes: u64,
    bookmarks: u64,
    profile_clicks: u64,
    url_clicks: u64,
}

impl Tableable for MetricsDisplay {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Metric", "Count"]);
        table.add_row(vec!["Tweet ID", &self.tweet_id]);
        table.add_row(vec!["Impressions", &self.impressions.to_string()]);
        table.add_row(vec!["Likes", &self.likes.to_string()]);
        table.add_row(vec!["Retweets", &self.retweets.to_string()]);
        table.add_row(vec!["Replies", &self.replies.to_string()]);
        table.add_row(vec!["Quotes", &self.quotes.to_string()]);
        table.add_row(vec!["Bookmarks", &self.bookmarks.to_string()]);
        table.add_row(vec!["Profile Clicks", &self.profile_clicks.to_string()]);
        table.add_row(vec!["URL Clicks", &self.url_clicks.to_string()]);
        table
    }
}

impl CsvRenderable for MetricsDisplay {
    fn csv_headers() -> Vec<&'static str> {
        vec!["tweet_id", "impressions", "likes", "retweets", "replies", "quotes", "bookmarks", "profile_clicks", "url_clicks"]
    }
    fn csv_rows(&self) -> Vec<Vec<String>> {
        vec![vec![
            self.tweet_id.clone(),
            self.impressions.to_string(),
            self.likes.to_string(),
            self.retweets.to_string(),
            self.replies.to_string(),
            self.quotes.to_string(),
            self.bookmarks.to_string(),
            self.profile_clicks.to_string(),
            self.url_clicks.to_string(),
        ]]
    }
}

fn oauth_secrets(ctx: &AppContext) -> reqwest_oauth1::Secrets<'_> {
    let k = &ctx.config.keys;
    reqwest_oauth1::Secrets::new(&k.api_key, &k.api_secret)
        .token(&k.access_token, &k.access_token_secret)
}

pub async fn execute(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    id: &str,
) -> Result<(), XmasterError> {
    if !ctx.config.has_x_auth() {
        return Err(XmasterError::AuthMissing {
            provider: "x",
            message: "X API credentials not configured".into(),
        });
    }

    let tweet_id = parse_tweet_id(id);
    let url = format!(
        "https://api.x.com/2/tweets/{tweet_id}?tweet.fields=public_metrics,non_public_metrics,organic_metrics"
    );

    let resp = ctx
        .client
        .clone()
        .oauth1(oauth_secrets(&ctx))
        .get(&url)
        .send()
        .await?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(XmasterError::Api {
            provider: "x",
            code: "api_error",
            message: format!("HTTP {status}: {text}"),
        });
    }

    let envelope: ApiEnvelope = resp.json().await?;
    let tweet = envelope.data.ok_or_else(|| XmasterError::NotFound(format!("Tweet {tweet_id}")))?;

    let public = tweet.public_metrics.unwrap_or_default();
    let non_public = tweet.non_public_metrics.unwrap_or_default();

    let display = MetricsDisplay {
        tweet_id: tweet.id,
        impressions: public.impression_count,
        likes: public.like_count,
        retweets: public.retweet_count,
        replies: public.reply_count,
        quotes: public.quote_count,
        bookmarks: public.bookmark_count,
        profile_clicks: non_public.user_profile_clicks,
        url_clicks: non_public.url_link_clicks,
    };
    output::render(format, &display, None);
    Ok(())
}
