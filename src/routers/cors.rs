use axum::http::{HeaderMap, StatusCode};

use crate::{app_type::HeaderResponse, utils::get_cors_header};

pub async fn cors_handler() -> HeaderResponse {
    let mut headers = HeaderMap::new();
    headers = get_cors_header(headers);
    (StatusCode::OK, headers, ())
}