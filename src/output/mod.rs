pub mod json;
pub mod table;

use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Table,
}

impl OutputFormat {
    pub fn detect(json_flag: bool) -> Self {
        if json_flag || !std::io::IsTerminal::is_terminal(&std::io::stdout()) {
            OutputFormat::Json
        } else {
            OutputFormat::Table
        }
    }
}

pub fn render<T: Serialize + Tableable>(
    format: OutputFormat,
    data: &T,
    metadata: Option<serde_json::Value>,
) {
    match format {
        OutputFormat::Json => json::render(data, metadata),
        OutputFormat::Table => table::render(data),
    }
}

pub fn render_error(format: OutputFormat, code: &str, message: &str, suggestion: &str) {
    match format {
        OutputFormat::Json => json::render_error(code, message, suggestion),
        OutputFormat::Table => {
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
