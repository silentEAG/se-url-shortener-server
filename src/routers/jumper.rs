use axum::{Extension, extract::Path};
use sqlx::Row;
use crate::model::state::AppState;

pub async fn jumper_handler(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> String {
    let sql = "SELECT `long_url` FROM url_shorter.url_info WHERE `is_deleted`=0 AND `mur_hash_code`=(?)";
    let conn = sqlx::query(sql)
        .bind(id)
        .fetch_one(&state.pool).await;
    if let Ok(row) = conn {
        row.get::<String, _>(0)
    }
    else {
        "Error".to_string()
    }
}