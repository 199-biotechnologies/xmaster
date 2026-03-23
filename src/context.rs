use crate::config::AppConfig;
use reqwest::Client;
use std::time::Duration;

pub struct AppContext {
    pub config: AppConfig,
    pub client: Client,
}

impl AppContext {
    pub fn new(config: AppConfig) -> Self {
        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(60))
            .tcp_nodelay(true)
            .timeout(Duration::from_secs(config.settings.timeout))
            .user_agent(format!("xmaster/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client");
        Self { config, client }
    }
}
