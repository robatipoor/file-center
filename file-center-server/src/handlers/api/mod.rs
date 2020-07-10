pub mod access;
pub mod account;
pub mod file;
use actix_web::{HttpResponse, Result};

pub async fn ping() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("pong \n"))
}

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("application is up \n"))
}
