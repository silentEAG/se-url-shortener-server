use super::{AppError, AppErrorType};

#[inline]
pub fn er_unknown() -> AppError {
    AppError { 
        message: "Unknown Error".to_string(),
        error_type: AppErrorType::UnknownError, 
        error_code: 1000,
    }
}

#[inline]
pub fn er_data_not_found() -> AppError {
    AppError { 
        message: "Data not found in database".to_string(),
        error_type: AppErrorType::DataNotFoundError, 
        error_code: 1001,
    }
}

#[inline]
pub fn er_insert() -> AppError {
    AppError { 
        message: "SQL Insert Error".to_string(),
        error_type: AppErrorType::InsertError, 
        error_code: 1002,
    }
}