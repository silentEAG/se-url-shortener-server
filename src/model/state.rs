use sqlx::MySql;
use super::conf::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<MySql>,
    pub app_config: AppConfig
}