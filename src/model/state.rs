use sqlx::MySql;


#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<MySql>,
    pub shorter_url_domain: String,
    // pub app_config: AppConfig
}