#![allow(dead_code)]
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LogicError {
    BadRequest(String),
    WebsocketError(String),
    InternalError(String),
}

impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogicError::BadRequest(ref msg) => {
                write!(f, "[BadRequest] {}", msg)
            }
            LogicError::WebsocketError(ref msg) => {
                write!(f, "[WebsocketError] {}", msg)
            }
            LogicError::InternalError(ref msg) => {
                write!(f, "[InternalError] {}", msg)
            }
        }
    }
}

impl Error for LogicError {}

impl IntoResponse for LogicError {
    fn into_response(self) -> Response {
        let body = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
