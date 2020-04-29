use crate::payloads::requests::{LoginRequest, RegisterRequest};
use crate::services::account_service;
use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    let result = account_service::register(req.into_inner(), &pool).await;
    match result {
        Ok(r) => Ok(HttpResponse::Ok().content_type("application/json").json(r)),
        Err(e) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(e.to_string())),
    }
}

pub async fn login(req: web::Json<LoginRequest>, pool: DataPoolSqlite) -> Result<HttpResponse> {
    let result = account_service::login(req.into_inner(), &pool).await;
    match result {
        Ok(r) => Ok(HttpResponse::Ok().content_type("application/json").json(r)),
        Err(e) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(e.to_string())),
    }
}

pub async fn update(_req: web::Json<LoginRequest>, _pool: DataPoolSqlite) -> Result<HttpResponse> {
    todo!()
}

pub async fn delete(_req: web::Json<LoginRequest>, _pool: DataPoolSqlite) -> Result<HttpResponse> {
    todo!()
}
