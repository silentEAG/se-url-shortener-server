use std::collections::HashMap;

use axum::{
    http::{HeaderMap, StatusCode},
    Json
};


pub type Result<T> = std::result::Result<T, crate::error::AppError>;
pub type HandlerResult<T> = self::Result<T>;
pub type HeaderResponse = (StatusCode, HeaderMap, ());
pub type RedirectResponse = (StatusCode, HeaderMap, ());
pub type HandlerRedirectResult = self::HandlerResult<RedirectResponse>;
pub type JsonResponse = (StatusCode, HeaderMap, Json<HashMap<String, String>>);
pub type HandlerJsonResult = self::HandlerResult<JsonResponse>;