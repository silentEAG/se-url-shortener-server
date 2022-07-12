use sqlx::{Pool, Postgres};


#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub shorter_url_domain: String,
}