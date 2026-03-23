use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::intel::tracker::PostTracker;
use crate::output::{self, OutputFormat};
use std::sync::Arc;

/// Generate a daily performance report.
pub async fn daily(
    _ctx: Arc<AppContext>,
    format: OutputFormat,
) -> Result<(), XmasterError> {
    let tracker = PostTracker::open()?;
    let report = tracker.generate_report("daily")?;

    if report.total_posts == 0 {
        output::render_error(
            format,
            "no_data",
            "No posts found in the last 24 hours",
            "Post and track content first: `xmaster post \"...\"` then `xmaster track run`",
        );
        return Ok(());
    }

    output::render(format, &report, None);
    Ok(())
}

/// Generate a weekly performance report.
pub async fn weekly(
    _ctx: Arc<AppContext>,
    format: OutputFormat,
) -> Result<(), XmasterError> {
    let tracker = PostTracker::open()?;
    let report = tracker.generate_report("weekly")?;

    if report.total_posts == 0 {
        output::render_error(
            format,
            "no_data",
            "No posts found in the last 7 days",
            "Post and track content first: `xmaster post \"...\"` then `xmaster track run`",
        );
        return Ok(());
    }

    output::render(format, &report, None);
    Ok(())
}
