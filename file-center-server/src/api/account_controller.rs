use crate::middlewares::authentication::get_user_id_from_request;
use crate::payloads::requests::{LoginRequest, RegisterRequest};
use crate::services::account_service;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use log::error;
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    match account_service::register(req.into_inner(), &pool).await {
        Ok(resp) => {
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(resp));
        }
        Err(e) => {
            error!("unsuccessful user register error message : {}", e);
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json("unsuccessful register"));
        }
    }
}

pub async fn login(req: web::Json<LoginRequest>, pool: DataPoolSqlite) -> Result<HttpResponse> {
    match account_service::login(req.into_inner(), &pool).await {
        Ok(resp) => {
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(resp));
        }
        Err(e) => {
            error!("login unsuccessful error message : {}", e);
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json("login unsuccessful"));
        }
    }
}

pub async fn update_account(
    _req: web::Json<LoginRequest>,
    req: HttpRequest,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    let _user_id = match get_user_id_from_request(&pool.clone(), req).await {
        Ok(id) => id,
        Err(e) => {
            error!("unautherized user message error : {}", e);
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json")
                .body("User not Autherized"));
        }
    };
    todo!()
}

pub async fn delete_account(
    _req: web::Json<LoginRequest>,
    req: HttpRequest,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    let _user_id = match get_user_id_from_request(&pool.clone(), req).await {
        Ok(id) => id,
        Err(e) => {
            error!("unautherized user message error : {}", e);
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json")
                .body("User not Autherized"));
        }
    };
    todo!()
}
