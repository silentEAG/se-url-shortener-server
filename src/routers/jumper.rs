use std::time::{SystemTime, UNIX_EPOCH};

use axum::{Extension, extract::Path, http::{HeaderMap, StatusCode}};
use log::debug;
use sqlx::Row;
use crate::error::method::*;
use crate::{model::state::AppState, app_type::{HandlerRedirectResult, RedirectResponse}};

pub fn redirect(url: &str) -> RedirectResponse {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::LOCATION, url.parse().unwrap());
    (StatusCode::FOUND, headers, ())
}

async fn log_in_database(state: &AppState, id: i32) {
    let time = SystemTime::now().duration_since(UNIX_EPOCH)
    .unwrap().as_secs().to_string();
    let sql = "UPDATE url_info 
        SET visit_count = visit_count + 1,
        latest_visit_at = $1
        WHERE id = $2";
    let _ = sqlx::query(sql)
        .bind(time)
        .bind(id)
        .execute(&state.pool).await;
}

pub async fn jumper_handler(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> HandlerRedirectResult {
    let sql = "SELECT id, long_url FROM url_info WHERE is_deleted=false AND mur_hash_code=$1";
    let conn = sqlx::query(sql)
        .bind(id)
        .fetch_one(&state.pool).await;
    if let Ok(row) = conn {
        let id = row.get::<i32, _>(0);
        let long_url = row.get::<String, _>(1);
        log_in_database(&state, id).await;
        Ok(redirect(&long_url))
    }
    else {
        if let Err(e) = conn {
            debug!("{}", e);
        }
        Err(er_unknown())
    }
}