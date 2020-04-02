pub mod access_controller;
pub mod account_controller;
pub mod file_controller;
use actix_web::*;

pub async fn ping() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("pong \n"))
}

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("application is run \n"))
}
