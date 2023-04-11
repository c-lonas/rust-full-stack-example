use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    ReqwestError(reqwest::Error),
    HttpStatus(reqwest::StatusCode),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            ApiError::HttpStatus(status) => write!(f, "Unexpected status code: {}", status),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> ApiError {
        ApiError::ReqwestError(err)
    }
}

impl From<reqwest::StatusCode> for ApiError {
    fn from(status: reqwest::StatusCode) -> ApiError {
        ApiError::HttpStatus(status)
    }
}
