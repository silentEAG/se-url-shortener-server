
pub mod method;

use std::collections::HashMap;

use axum::{response::IntoResponse, http::StatusCode, Json};

#[derive(Debug)]
pub enum AppErrorType {
    DBError,
    DataNotFoundError,
    UnknownError,
    InsertError
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub error_type: AppErrorType,
    pub error_code: i32
}

impl AppError {
    // pub fn err_type(&self) -> AppErrorType {
    //     self.error_type
    // }
    pub fn new (message: String) -> Self {
        Self {
            message,
            error_type: AppErrorType::UnknownError,
            error_code: 1000
        }
    }
    pub fn db_error(err: impl ToString) -> Self {
        Self {
            message: err.to_string(),
            error_type: AppErrorType::DBError,
            error_code: 1001
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::db_error(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let mut json_body: HashMap<String, String> = HashMap::new();
        json_body.insert("error_message".to_string(), self.message);
        json_body.insert("error_code".to_string(), self.error_code.to_string());
        (StatusCode::INTERNAL_SERVER_ERROR, Json::from(json_body)).into_response()
    }
}