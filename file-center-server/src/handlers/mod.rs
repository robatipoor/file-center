pub mod api;
use crate::models::user::UserAuth;
use crate::models::DataPoolSqlite;
use crate::services::file::list_files_service;
use actix_web::*;
use actix_web::{web, HttpResponse, Result};
use log::error;
use tera::{Context, Tera};

// register page
pub async fn register_page(tml: web::Data<Tera>) -> Result<HttpResponse> {
    let data = tml
        .render("register.html", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(data))
}

// home page
pub async fn home_page(tml: web::Data<Tera>) -> Result<HttpResponse> {
    let data = tml
        .render("home.html", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(data))
}

pub async fn login_page(tml: web::Data<Tera>) -> Result<HttpResponse> {
    let s = tml
        .render("login.html", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn upload_page(tml: web::Data<Tera>) -> Result<HttpResponse> {
    let s = tml
        .render("upload.html", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn list_file_page(
    pool: DataPoolSqlite,
    tml: web::Data<Tera>,
    user_auth: UserAuth,
) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    match list_files_service(&pool, user_auth.id).await {
        Ok(r) => {
            ctx.insert("files", &r);
        }
        Err(e) => {
            error!("{}", e);
        }
    }
    let data = tml
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(data))
}
