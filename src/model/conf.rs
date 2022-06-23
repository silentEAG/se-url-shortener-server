use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sql_config: Option<SqlConfig>,
    pub app_config: Option<AppConfig>
}
#[derive(Debug, Deserialize)]
pub struct SqlConfig {
    pub host: Option<String>,
    pub port: Option<String>,
    pub max_connections: Option<u32>
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub host: Option<String>,
    pub port: Option<String>,
    pub shorter_url_domain: Option<String>
}

impl SqlConfig {
    pub fn default_config() -> Self {
        SqlConfig { 
            host: Some("127.0.0.1".to_string()), 
            port: Some("3306".to_string()),
            max_connections: Some(5)
        }
    }
}