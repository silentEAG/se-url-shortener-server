use axum::{
    http::{HeaderMap, StatusCode},
};


pub type Result<T> = std::result::Result<T, crate::error::AppError>;
pub type HandlerResult<T> = self::Result<T>;
pub type RedirectResponse = (StatusCode, HeaderMap, ());
pub type HandlerRedirectResult = self::HandlerResult<RedirectResponse>;