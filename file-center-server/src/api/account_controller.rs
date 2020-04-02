use crate::payloads::requests::{LoginRequest, RegisterRequest};
use crate::services::account_service;
use actix_web::{web, HttpResponse, Result};
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;
type ResutResponse = Result<HttpResponse>;

pub async fn register(req: web::Json<RegisterRequest>, pool: DataPoolSqlite) -> ResutResponse {
    let re = account_service::register(req.into_inner(), &pool).await;
    match re {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => Ok(HttpResponse::Ok().json(e.to_string())),
    }
}

pub async fn login(req: web::Json<LoginRequest>, pool: DataPoolSqlite) -> ResutResponse {
    let re = account_service::login(req.into_inner(), &pool).await;
    match re {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => Ok(HttpResponse::Ok().json(e.to_string())),
    }
}
