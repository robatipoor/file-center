use crate::models::user::UserAuth;
use crate::payloads::requests::{LoginRequest, RegisterRequest};
use crate::services::account::*;
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Result};
use log::error;
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    match register_service(req.into_inner(), &pool).await {
        Ok(resp) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(resp)),
        Err(e) => {
            error!("unsuccessful user register error message : {}", e);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json("unsuccessful register"))
        }
    }
}

pub async fn login(
    req: web::Json<LoginRequest>,
    _identity: Identity,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    match login_service(req.into_inner(), &pool).await {
        Ok(resp) => {
            // identity.remember("jwt".to_owned());
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(resp))
        }
        Err(e) => {
            error!("login unsuccessful error message : {}", e);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json("login unsuccessful"))
        }
    }
}

pub async fn logout(
    _req: web::Json<LoginRequest>,
    _user_auth: UserAuth,
    identity: Identity,
    _pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    identity.forget();
    todo!()
}

pub async fn update_account(
    _req: web::Json<LoginRequest>,
    _user_auth: UserAuth,
    _pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    todo!()
}

pub async fn delete_account(
    _req: web::Json<LoginRequest>,
    _user_auth: UserAuth,
    _pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    todo!()
}
