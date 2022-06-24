use axum::{response::IntoResponse, http::StatusCode};


#[derive(Debug)]
pub enum AppErrorType {
    DBError,
    NotFound
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub error_type: AppErrorType,
    pub error_code: i32
}

impl AppError {
    pub fn new (message: String, error_type: AppErrorType, error_code: i32) -> Self {
        Self {
            message,
            error_type,
            error_code
        }
    }
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "ERROR").into_response()
    }
}