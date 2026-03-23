use crate::config::{self, AppConfig};
use crate::context::AppContext;
use crate::errors::XmasterError;
use crate::output::{self, OutputFormat, Tableable};
use crate::providers::xapi::XApi;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct ConfigDisplay {
    config_path: String,
    api_key: String,
    api_secret: String,
    access_token: String,
    access_token_secret: String,
    bearer_token: String,
    xai_key: String,
    timeout: u64,
    default_count: usize,
}

impl Tableable for ConfigDisplay {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Setting", "Value"]);
        table.add_row(vec!["Config path", &self.config_path]);
        table.add_row(vec!["API Key", &self.api_key]);
        table.add_row(vec!["API Secret", &self.api_secret]);
        table.add_row(vec!["Access Token", &self.access_token]);
        table.add_row(vec!["Access Token Secret", &self.access_token_secret]);
        table.add_row(vec!["Bearer Token", &self.bearer_token]);
        table.add_row(vec!["xAI Key", &self.xai_key]);
        table.add_row(vec!["Timeout (s)", &self.timeout.to_string()]);
        table.add_row(vec!["Default Count", &self.default_count.to_string()]);
        table
    }
}

#[derive(Serialize)]
struct ConfigSetResult {
    key: String,
    success: bool,
}

impl Tableable for ConfigSetResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Key", "Status"]);
        table.add_row(vec![
            self.key.as_str(),
            if self.success { "Updated" } else { "Failed" },
        ]);
        table
    }
}

#[derive(Serialize)]
struct ConfigCheckResult {
    x_auth: AuthStatus,
    xai_auth: AuthStatus,
}

#[derive(Serialize)]
struct AuthStatus {
    configured: bool,
    valid: bool,
    detail: String,
}

impl Tableable for ConfigCheckResult {
    fn to_table(&self) -> comfy_table::Table {
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Provider", "Configured", "Valid", "Detail"]);
        table.add_row(vec![
            "X API",
            if self.x_auth.configured { "Yes" } else { "No" },
            if self.x_auth.valid { "Yes" } else { "No" },
            &self.x_auth.detail,
        ]);
        table.add_row(vec![
            "xAI",
            if self.xai_auth.configured { "Yes" } else { "No" },
            if self.xai_auth.valid { "Yes" } else { "No" },
            &self.xai_auth.detail,
        ]);
        table
    }
}

fn mask(key: &str) -> String {
    if key.is_empty() {
        "(not set)".into()
    } else {
        AppConfig::masked_key(key)
    }
}

pub async fn show(_ctx: Arc<AppContext>, format: OutputFormat) -> Result<(), XmasterError> {
    let cfg = config::load_config()?;
    let display = ConfigDisplay {
        config_path: config::config_path().to_string_lossy().to_string(),
        api_key: mask(&cfg.keys.api_key),
        api_secret: mask(&cfg.keys.api_secret),
        access_token: mask(&cfg.keys.access_token),
        access_token_secret: mask(&cfg.keys.access_token_secret),
        bearer_token: mask(&cfg.keys.bearer_token),
        xai_key: mask(&cfg.keys.xai),
        timeout: cfg.settings.timeout,
        default_count: cfg.settings.count,
    };
    output::render(format, &display, None);
    Ok(())
}

pub async fn set(format: OutputFormat, key: &str, value: &str) -> Result<(), XmasterError> {
    let path = config::config_path();

    // Read existing TOML or start fresh
    let existing = if path.exists() {
        std::fs::read_to_string(&path).unwrap_or_default()
    } else {
        // Ensure config directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        String::new()
    };

    let mut doc: toml::Table = existing
        .parse()
        .map_err(|e: toml::de::Error| XmasterError::Config(format!("Failed to parse config: {e}")))?;

    // Parse key path like "keys.api_key" → ["keys", "api_key"]
    let parts: Vec<&str> = key.split('.').collect();
    match parts.len() {
        1 => {
            doc.insert(parts[0].to_string(), toml::Value::String(value.to_string()));
        }
        2 => {
            let section = doc
                .entry(parts[0].to_string())
                .or_insert_with(|| toml::Value::Table(toml::Table::new()));
            if let toml::Value::Table(ref mut t) = section {
                t.insert(parts[1].to_string(), toml::Value::String(value.to_string()));
            }
        }
        _ => {
            return Err(XmasterError::Config(format!("Invalid key path: {key}")));
        }
    }

    let toml_str = toml::to_string_pretty(&doc)
        .map_err(|e| XmasterError::Config(format!("Failed to serialize config: {e}")))?;
    std::fs::write(&path, toml_str)?;

    let display = ConfigSetResult {
        key: key.to_string(),
        success: true,
    };
    output::render(format, &display, None);
    Ok(())
}

pub async fn check(ctx: Arc<AppContext>, format: OutputFormat) -> Result<(), XmasterError> {
    let x_configured = ctx.config.has_x_auth();
    let xai_configured = ctx.config.has_xai_auth();

    let x_auth = if x_configured {
        let api = XApi::new(ctx.clone());
        match api.get_me().await {
            Ok(user) => AuthStatus {
                configured: true,
                valid: true,
                detail: format!("Authenticated as @{}", user.username),
            },
            Err(e) => AuthStatus {
                configured: true,
                valid: false,
                detail: format!("Auth failed: {e}"),
            },
        }
    } else {
        AuthStatus {
            configured: false,
            valid: false,
            detail: "X API credentials not set".into(),
        }
    };

    let xai_auth = AuthStatus {
        configured: xai_configured,
        valid: xai_configured,
        detail: if xai_configured {
            "xAI API key configured".into()
        } else {
            "xAI API key not set".into()
        },
    };

    let display = ConfigCheckResult { x_auth, xai_auth };
    output::render(format, &display, None);
    Ok(())
}
