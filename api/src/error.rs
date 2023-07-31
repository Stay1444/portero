use std::fmt::Display;

use axum::{
    response::{
        Response, 
        IntoResponse
    }, 
    http::StatusCode
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    SerializationFail,
    DeserializationFail,
    DatabaseError
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LoginFail => write!(f, "LOGIN_FAIL"),
            Error::SerializationFail => write!(f, "SERIALIZATION_FAIL"),
            Error::DeserializationFail => write!(f, "DESERIALIZATION_FAIL"),
            Error::DatabaseError => write!(f, "DATABASE_ERROR"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_ERROR").into_response()
    }
}