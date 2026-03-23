use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::intel::preflight::{self, PreflightResult, Severity};
use crate::output::{self, OutputFormat, Tableable};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct AnalyzeDisplay {
    #[serde(flatten)]
    result: PreflightResult,
}

impl Tableable for AnalyzeDisplay {
    fn to_table(&self) -> comfy_table::Table {
        use comfy_table::{Attribute, Cell, Color};

        let mut table = comfy_table::Table::new();

        // Header section: Score + Grade
        table.set_header(vec!["Field", "Value"]);

        let grade_color = match self.result.grade.as_str() {
            "A" => Color::Green,
            "B" => Color::Cyan,
            "C" => Color::Yellow,
            "D" => Color::Red,
            _ => Color::DarkRed,
        };

        table.add_row(vec![
            Cell::new("Score"),
            Cell::new(format!("{}/100", self.result.score)).fg(grade_color),
        ]);
        table.add_row(vec![
            Cell::new("Grade"),
            Cell::new(&self.result.grade)
                .fg(grade_color)
                .add_attribute(Attribute::Bold),
        ]);
        table.add_row(vec![
            Cell::new("Type"),
            Cell::new(&self.result.features.content_type_guess),
        ]);
        table.add_row(vec![
            Cell::new("Characters"),
            Cell::new(format!(
                "{}/280",
                self.result.features.char_count
            )),
        ]);
        table.add_row(vec![
            Cell::new("Hook Strength"),
            Cell::new(format!("{}/100", self.result.features.hook_strength)),
        ]);

        // Issues section
        if !self.result.issues.is_empty() {
            table.add_row(vec![
                Cell::new("").add_attribute(Attribute::Dim),
                Cell::new("--- Issues ---").add_attribute(Attribute::Dim),
            ]);
            for issue in &self.result.issues {
                let severity_color = match issue.severity {
                    Severity::Critical => Color::Red,
                    Severity::Warning => Color::Yellow,
                    Severity::Info => Color::Cyan,
                };
                table.add_row(vec![
                    Cell::new(format!("[{}]", issue.severity)).fg(severity_color),
                    Cell::new(&issue.message),
                ]);
            }
        }

        // Suggestions section
        if !self.result.suggestions.is_empty() {
            table.add_row(vec![
                Cell::new("").add_attribute(Attribute::Dim),
                Cell::new("--- Suggestions ---").add_attribute(Attribute::Dim),
            ]);
            for (i, suggestion) in self.result.suggestions.iter().enumerate() {
                table.add_row(vec![
                    Cell::new(format!("{}.", i + 1)),
                    Cell::new(suggestion),
                ]);
            }
        }

        // Next command
        if !self.result.suggested_next_commands.is_empty() {
            table.add_row(vec![
                Cell::new("").add_attribute(Attribute::Dim),
                Cell::new("--- Next ---").add_attribute(Attribute::Dim),
            ]);
            for cmd in &self.result.suggested_next_commands {
                table.add_row(vec![
                    Cell::new("Run"),
                    Cell::new(cmd).fg(Color::Green),
                ]);
            }
        }

        table
    }
}

pub async fn execute(
    _ctx: Arc<AppContext>,
    format: OutputFormat,
    text: &str,
    goal: Option<&str>,
) -> Result<(), XmasterError> {
    let result = preflight::analyze(text, goal);
    let display = AnalyzeDisplay { result };
    output::render(format, &display, None);
    Ok(())
}
