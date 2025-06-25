use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
    pub status: u16,
}

impl<T> ApiResponse<T> {
    pub fn new(data: Option<T>, message: String, status: u16) -> Self {
        Self {
            data,
            message,
            status,
        }
    }

    pub fn ok(message: String, data: Option<T>) -> Self {
        Self {
            data,
            message,
            status: 0,
        }
    }

    pub fn err(message: String) -> Self {
        Self {
            data: None,
            message,
            status: 1,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
