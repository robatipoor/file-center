use actix_web::{http::StatusCode, HttpResponse};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, new)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize, new)]
pub struct ResponseBody<T> {
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, new)]
pub struct ServiceError {
    pub status: StatusCode,
    pub message: String,
}

impl ServiceError {
    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(&self.message)
    }
}
