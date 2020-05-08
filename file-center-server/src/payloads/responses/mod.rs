use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum Status {
    SUCCESS,
    UNSUCCESS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
}

impl TokenResponse {
    pub fn new(token: &str) -> Self {
        TokenResponse {
            token: token.to_string(),
            token_type: "bearer".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub status: Status,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ResponseBody<T>
where
    T: Serialize,
{
    pub fn new(status: Status, msg: &str) -> Self {
        ResponseBody {
            status,
            message: msg.to_string(),
            data: None,
        }
    }

    pub fn add_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn to_response(&self, status: StatusCode) -> HttpResponse {
        HttpResponse::build(status).json(self)
    }
}
