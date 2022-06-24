use axum::{Extension, extract::Path, http::{HeaderMap, StatusCode}};
use sqlx::Row;
use crate::{model::state::AppState, app_type::{HandlerRedirectResult, RedirectResponse}};

pub fn redirect(url: &str) -> RedirectResponse {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::LOCATION, url.parse().unwrap());
    (StatusCode::FOUND, headers, ())
}

pub async fn jumper_handler(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> HandlerRedirectResult {

    let sql = "SELECT `long_url` FROM url_shorter.url_info WHERE `is_deleted`=0 AND `mur_hash_code`=(?)";
    let conn = sqlx::query(sql)
        .bind(id)
        .fetch_one(&state.pool).await;
    let mut long_url = "https://silente.top".to_string();
    if let Ok(row) = conn {
        long_url = row.get::<String, _>(0);
    }
    Ok(redirect(&long_url))
}