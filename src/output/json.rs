use serde::Serialize;

pub fn render<T: Serialize>(data: &T, metadata: Option<serde_json::Value>) {
    let envelope = serde_json::json!({
        "version": "1",
        "status": "success",
        "data": data,
        "metadata": metadata.unwrap_or(serde_json::json!({})),
    });
    println!("{}", serde_json::to_string_pretty(&envelope).unwrap_or_default());
}

pub fn render_error(code: &str, message: &str, suggestion: &str) {
    let envelope = serde_json::json!({
        "version": "1",
        "status": "error",
        "error": {
            "code": code,
            "message": message,
            "suggestion": suggestion,
        },
    });
    // Errors go to stderr per agent-cli-framework invariant 6:
    // `tool cmd | jq` must never see error JSON on stdout.
    eprintln!("{}", serde_json::to_string_pretty(&envelope).unwrap_or_default());
}
