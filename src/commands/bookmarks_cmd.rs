use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::intel::bookmarks::{BookmarkRecord, BookmarkStore};
use crate::output::{self, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Display types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct BookmarkList {
    bookmarks: Vec<BookmarkRow>,
    total: usize,
}

#[derive(Serialize)]
struct BookmarkRow {
    id: String,
    author: String,
    text: String,
    likes: i64,
    saved: String,
}

impl Tableable for BookmarkList {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["ID", "Author", "Text", "Likes", "Saved"]);
        for b in &self.bookmarks {
            let truncated = if b.text.len() > 60 {
                format!("{}...", &b.text[..57])
            } else {
                b.text.clone()
            };
            table.add_row(vec![
                &b.id,
                &b.author,
                &truncated,
                &b.likes.to_string(),
                &b.saved,
            ]);
        }
        table
    }
}

#[derive(Serialize)]
struct SyncDisplay {
    new_bookmarks: u32,
    already_stored: u32,
    total_in_db: u32,
    message: String,
}

impl Tableable for SyncDisplay {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Field", "Value"]);
        table.add_row(vec!["New", &self.new_bookmarks.to_string()]);
        table.add_row(vec!["Already stored", &self.already_stored.to_string()]);
        table.add_row(vec!["Total in archive", &self.total_in_db.to_string()]);
        table.add_row(vec!["Status", &self.message]);
        table
    }
}

#[derive(Serialize)]
struct ExportDisplay {
    count: usize,
    output: String,
    message: String,
}

impl Tableable for ExportDisplay {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Field", "Value"]);
        table.add_row(vec!["Exported", &self.count.to_string()]);
        table.add_row(vec!["Output", &self.output]);
        table.add_row(vec!["Status", &self.message]);
        table
    }
}

#[derive(Serialize)]
struct DigestDisplay {
    period_days: u32,
    count: u32,
    unique_authors: usize,
    link_count: u32,
    text_count: u32,
    top_authors: Vec<AuthorSummary>,
}

#[derive(Serialize)]
struct AuthorSummary {
    username: String,
    count: u32,
}

impl Tableable for DigestDisplay {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Field", "Value"]);
        table.add_row(vec![
            "Period",
            &format!("Last {} days", self.period_days),
        ]);
        table.add_row(vec!["Bookmarks", &self.count.to_string()]);
        table.add_row(vec!["Authors", &self.unique_authors.to_string()]);
        table.add_row(vec!["With links", &self.link_count.to_string()]);
        table.add_row(vec!["Text only", &self.text_count.to_string()]);
        for a in &self.top_authors {
            table.add_row(vec![
                &format!("@{}", a.username),
                &format!("{} bookmarks", a.count),
            ]);
        }
        table
    }
}

#[derive(Serialize)]
struct StatsDisplay {
    total: u32,
    unread: u32,
    with_links: u32,
    with_media: u32,
    top_authors: Vec<(String, u32)>,
    oldest: Option<String>,
    newest: Option<String>,
}

impl Tableable for StatsDisplay {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Field", "Value"]);
        table.add_row(vec!["Total", &self.total.to_string()]);
        table.add_row(vec!["Unread", &self.unread.to_string()]);
        table.add_row(vec!["With links", &self.with_links.to_string()]);
        table.add_row(vec!["With media", &self.with_media.to_string()]);
        if let Some(ref o) = self.oldest {
            table.add_row(vec!["Oldest", o]);
        }
        if let Some(ref n) = self.newest {
            table.add_row(vec!["Newest", n]);
        }
        for (author, count) in &self.top_authors {
            table.add_row(vec![
                &format!("@{author}"),
                &format!("{count} bookmarks"),
            ]);
        }
        table
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn records_to_list(records: Vec<BookmarkRecord>) -> BookmarkList {
    let total = records.len();
    let bookmarks = records
        .into_iter()
        .map(|r| BookmarkRow {
            id: r.tweet_id,
            author: format!("@{}", r.author_username),
            text: r.text,
            likes: r.likes,
            saved: r.bookmarked_at[..10].to_string(),
        })
        .collect();
    BookmarkList { bookmarks, total }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

pub async fn list(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    count: usize,
    unread: bool,
) -> Result<(), XmasterError> {
    if unread {
        let store = BookmarkStore::open()?;
        let records = store.list_unread(count)?;
        if records.is_empty() {
            output::render_error(
                format,
                "no_unread_bookmarks",
                "No unread bookmarks found",
                "Sync bookmarks first: xmaster bookmarks sync",
            );
            return Ok(());
        }
        output::render(format, &records_to_list(records), None);
    } else {
        // Live from X API
        let api = XApi::new(ctx.clone());
        let tweets = api.get_bookmarks(count).await?;
        let records: Vec<BookmarkRecord> = tweets
            .into_iter()
            .map(|t| {
                let metrics = t.public_metrics.as_ref();
                BookmarkRecord {
                    tweet_id: t.id,
                    author_username: t
                        .author_username
                        .unwrap_or_else(|| t.author_id.unwrap_or_default()),
                    author_name: None,
                    text: t.text,
                    created_at: t.created_at.clone(),
                    bookmarked_at: t.created_at.unwrap_or_default(),
                    likes: metrics.map(|m| m.like_count as i64).unwrap_or(0),
                    retweets: metrics.map(|m| m.retweet_count as i64).unwrap_or(0),
                    replies: metrics.map(|m| m.reply_count as i64).unwrap_or(0),
                    has_media: false,
                    has_link: false,
                    tags: String::new(),
                    notes: String::new(),
                    read: false,
                }
            })
            .collect();
        output::render(format, &records_to_list(records), None);
    }
    Ok(())
}

pub async fn sync(
    ctx: Arc<AppContext>,
    format: OutputFormat,
    count: usize,
) -> Result<(), XmasterError> {
    let api = XApi::new(ctx.clone());
    let tweets = api.get_bookmarks(count).await?;
    let store = BookmarkStore::open()?;
    let result = store.sync(tweets)?;

    let display = SyncDisplay {
        new_bookmarks: result.new_bookmarks,
        already_stored: result.already_stored,
        total_in_db: result.total_in_db,
        message: format!(
            "Synced: {} new, {} already stored. Total: {} in local archive",
            result.new_bookmarks, result.already_stored, result.total_in_db
        ),
    };
    output::render(format, &display, None);

    if format == OutputFormat::Table {
        eprintln!(
            "Search: xmaster bookmarks search \"query\"",
        );
        eprintln!(
            "Export: xmaster bookmarks export -o bookmarks.md",
        );
    }
    Ok(())
}

pub async fn search(format: OutputFormat, query: &str) -> Result<(), XmasterError> {
    let store = BookmarkStore::open()?;
    let records = store.search(query)?;

    if records.is_empty() {
        output::render_error(
            format,
            "no_results",
            &format!("No bookmarks matching '{query}'"),
            "Try a broader search term or sync more bookmarks: xmaster bookmarks sync",
        );
        return Ok(());
    }

    output::render(format, &records_to_list(records), None);
    Ok(())
}

pub async fn export(
    format: OutputFormat,
    output_path: Option<&str>,
    unread: bool,
) -> Result<(), XmasterError> {
    let store = BookmarkStore::open()?;

    let records = if unread {
        store.list_unread(1000)?
    } else {
        store.search("")? // get all
    };

    if records.is_empty() {
        output::render_error(
            format,
            "no_bookmarks",
            "No bookmarks to export",
            "Sync bookmarks first: xmaster bookmarks sync",
        );
        return Ok(());
    }

    let count = records.len();
    let md = BookmarkStore::export_markdown(&records);

    // Mark exported bookmarks as read
    for r in &records {
        store.mark_read(&r.tweet_id)?;
    }

    let output_desc = match output_path {
        Some(path) => {
            std::fs::write(path, &md)?;
            path.to_string()
        }
        None => {
            println!("{md}");
            "stdout".to_string()
        }
    };

    if output_path.is_some() || format == OutputFormat::Json {
        let display = ExportDisplay {
            count,
            output: output_desc,
            message: format!("Exported {count} bookmarks (marked as read)"),
        };
        output::render(format, &display, None);
    }
    Ok(())
}

pub async fn digest(format: OutputFormat, days: u32) -> Result<(), XmasterError> {
    let store = BookmarkStore::open()?;
    let digest = store.get_digest(days)?;

    if digest.count == 0 {
        output::render_error(
            format,
            "no_bookmarks_in_period",
            &format!("No bookmarks in the last {days} days"),
            "Sync bookmarks first: xmaster bookmarks sync",
        );
        return Ok(());
    }

    let display = DigestDisplay {
        period_days: digest.period_days,
        count: digest.count,
        unique_authors: digest.by_author.len(),
        link_count: digest.link_count,
        text_count: digest.text_count,
        top_authors: digest
            .by_author
            .iter()
            .take(10)
            .map(|a| AuthorSummary {
                username: a.username.clone(),
                count: a.count,
            })
            .collect(),
    };
    output::render(format, &display, None);
    Ok(())
}

pub async fn stats(format: OutputFormat) -> Result<(), XmasterError> {
    let store = BookmarkStore::open()?;
    let stats = store.get_stats()?;

    if stats.total == 0 {
        output::render_error(
            format,
            "no_bookmarks",
            "No bookmarks in local database",
            "Sync bookmarks first: xmaster bookmarks sync -c 200",
        );
        return Ok(());
    }

    let display = StatsDisplay {
        total: stats.total,
        unread: stats.unread,
        with_links: stats.with_links,
        with_media: stats.with_media,
        top_authors: stats.top_authors,
        oldest: stats.oldest,
        newest: stats.newest,
    };
    output::render(format, &display, None);
    Ok(())
}
