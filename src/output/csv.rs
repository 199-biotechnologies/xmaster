use serde::Serialize;

/// Trait for types that can be rendered as CSV rows.
///
/// A default implementation is provided that serializes the value as a single
/// JSON cell — types with tabular data should override with proper columns.
pub trait CsvRenderable {
    fn csv_headers() -> Vec<&'static str> {
        vec!["data"]
    }

    fn csv_rows(&self) -> Vec<Vec<String>>
    where
        Self: Serialize,
    {
        // Fallback: emit the whole value as a single JSON cell.
        let json = serde_json::to_string(self).unwrap_or_default();
        vec![vec![json]]
    }
}

/// Escape a CSV field: quote if it contains commas, quotes, or newlines.
fn escape_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub fn render<T: Serialize + CsvRenderable>(data: &T, _metadata: Option<serde_json::Value>) {
    let headers = T::csv_headers();
    println!(
        "{}",
        headers
            .iter()
            .map(|h| escape_field(h))
            .collect::<Vec<_>>()
            .join(",")
    );
    for row in data.csv_rows() {
        println!(
            "{}",
            row.iter()
                .map(|f| escape_field(f))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}
