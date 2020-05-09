pub mod access;
pub mod account;
pub mod file;
use actix_web::*;
use actix_identity::Identity;

pub async fn ping(id :Identity) -> Result<HttpResponse> {
    println!("id = > {:?}",id.identity());
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("pong \n"))
}

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("application is up \n"))
}
