use crate::payloads::requests::{LoginRequest, RegisterRequest};
use crate::services::account_service;
use actix_web::{web, HttpResponse, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;
type ResutResponse = Result<HttpResponse>;

pub async fn register(req: web::Json<RegisterRequest>, pool: PoolSqliteData) -> ResutResponse {
    let re = account_service::register(req.into_inner(), pool);
    match re {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => Ok(e.response()),
    }
}

pub async fn login(req: web::Json<LoginRequest>, pool: PoolSqliteData) -> ResutResponse {
    let re = account_service::login(req.into_inner(), pool);
    match re {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => Ok(e.response()),
    }
}
