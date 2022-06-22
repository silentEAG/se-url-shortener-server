use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sql_config: Option<SqlConfig>,
    pub app_config: Option<AppConfig>
}
#[derive(Debug, Deserialize)]
pub struct SqlConfig {
    pub host: Option<String>,
    pub port: Option<u32>,
    pub max_connections: Option<u32>
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub domain: Option<String>
}

impl SqlConfig {
    pub fn default_config() -> Self {
        SqlConfig { 
            host: Some("127.0.0.1".to_string()), 
            port: Some(3306),
            max_connections: Some(5)
    }
    }
}