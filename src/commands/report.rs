use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::intel::tracker::PostTracker;
use crate::output::{self, OutputFormat, Tableable};
use serde::Serialize;
use std::sync::Arc;

/// Pattern observations generated from the report data.
#[derive(Serialize)]
struct PatternObservation {
    pattern: String,
    lift: String,
}

#[derive(Serialize)]
struct EnhancedReport {
    #[serde(flatten)]
    report: crate::intel::tracker::PerformanceReport,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pattern_observations: Vec<PatternObservation>,
}

impl Tableable for EnhancedReport {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = self.report.to_table();
        for obs in &self.pattern_observations {
            table.add_row(vec!["Pattern", &format!("{} ({})", obs.pattern, obs.lift)]);
        }
        table
    }
}

/// Derive pattern observations from the content breakdown.
fn derive_patterns(report: &crate::intel::tracker::PerformanceReport) -> Vec<PatternObservation> {
    let mut observations = Vec::new();

    if report.content_breakdown.len() < 2 || report.avg_engagement_rate == 0.0 {
        return observations;
    }

    // Find content types that significantly outperform the average
    for ct in &report.content_breakdown {
        if ct.count < 2 {
            continue;
        }
        let lift = ct.avg_engagement_rate / report.avg_engagement_rate;
        if lift > 1.2 {
            observations.push(PatternObservation {
                pattern: format!("'{}' posts outperform average", ct.content_type),
                lift: format!("{:.1}x engagement rate", lift),
            });
        } else if lift < 0.7 {
            observations.push(PatternObservation {
                pattern: format!("'{}' posts underperform average", ct.content_type),
                lift: format!("{:.1}x engagement rate", lift),
            });
        }
    }

    // Compare best vs worst post if both exist
    if let (Some(best), Some(worst)) = (&report.best_post, &report.worst_post) {
        if worst.engagement_rate > 0.0 {
            let ratio = best.engagement_rate / worst.engagement_rate;
            if ratio > 3.0 {
                observations.push(PatternObservation {
                    pattern: "Wide variance between best and worst posts".into(),
                    lift: format!("{:.0}x spread — experiment more with what works", ratio),
                });
            }
        }
    }

    observations
}

fn render_report(tracker: &PostTracker, period: &str, format: OutputFormat) -> Result<(), XmasterError> {
    let report = tracker.generate_report(period)?;

    if report.total_posts == 0 {
        let label = match period {
            "daily" => "24 hours",
            "weekly" => "7 days",
            "monthly" => "30 days",
            _ => "selected period",
        };
        return Err(XmasterError::NotFound(
            format!("No posts found in the last {label}. Post and track content first: `xmaster post \"...\"` then `xmaster track run`"),
        ));
    }

    let patterns = derive_patterns(&report);
    let enhanced = EnhancedReport {
        report,
        pattern_observations: patterns,
    };
    output::render(format, &enhanced, None);
    Ok(())
}

/// Generate a daily performance report.
pub async fn daily(
    _ctx: Arc<AppContext>,
    format: OutputFormat,
) -> Result<(), XmasterError> {
    let tracker = PostTracker::open()?;
    render_report(&tracker, "daily", format)
}

/// Generate a weekly performance report.
pub async fn weekly(
    _ctx: Arc<AppContext>,
    format: OutputFormat,
) -> Result<(), XmasterError> {
    let tracker = PostTracker::open()?;
    render_report(&tracker, "weekly", format)
}
