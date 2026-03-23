pub mod csv;
pub mod json;
pub mod table;

use serde::Serialize;

pub use csv::CsvRenderable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Table,
    Csv,
}

impl OutputFormat {
    /// Detect output format from CLI flags. The single-argument form is kept for
    /// backward compatibility — callers that have a CSV flag should use `detect_full`.
    pub fn detect(json_flag: bool) -> Self {
        Self::detect_full(json_flag, false)
    }

    pub fn detect_full(json_flag: bool, csv_flag: bool) -> Self {
        if csv_flag {
            OutputFormat::Csv
        } else if json_flag || !std::io::IsTerminal::is_terminal(&std::io::stdout()) {
            OutputFormat::Json
        } else {
            OutputFormat::Table
        }
    }
}

/// Render data in the selected format. CSV falls back to JSON for types that
/// only carry the default (single-cell) CsvRenderable implementation.
pub fn render<T: Serialize + Tableable>(
    format: OutputFormat,
    data: &T,
    metadata: Option<serde_json::Value>,
) {
    match format {
        OutputFormat::Json => json::render(data, metadata),
        OutputFormat::Table => table::render(data),
        // Types without a dedicated CsvRenderable impl will hit this branch
        // and get JSON output instead — CSV is opt-in per type.
        OutputFormat::Csv => json::render(data, metadata),
    }
}

/// Render data with full CSV support. Use this in commands whose display types
/// implement CsvRenderable with proper column definitions.
pub fn render_csv<T: Serialize + Tableable + CsvRenderable>(
    format: OutputFormat,
    data: &T,
    metadata: Option<serde_json::Value>,
) {
    match format {
        OutputFormat::Json => json::render(data, metadata),
        OutputFormat::Table => table::render(data),
        OutputFormat::Csv => csv::render(data, metadata),
    }
}

pub fn render_error(format: OutputFormat, code: &str, message: &str, suggestion: &str) {
    match format {
        OutputFormat::Json => json::render_error(code, message, suggestion),
        OutputFormat::Table | OutputFormat::Csv => {
            eprintln!("Error: {message}");
            if !suggestion.is_empty() {
                eprintln!("  Suggestion: {suggestion}");
            }
        }
    }
}

pub trait Tableable {
    fn to_table(&self) -> comfy_table::Table;
}
