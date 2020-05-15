pub mod access;
pub mod account;
pub mod file;
use actix_web::{HttpResponse, Result};

pub async fn ping() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("pong \n"))
}

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("application is up \n"))
}
