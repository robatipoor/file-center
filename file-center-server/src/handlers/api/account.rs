use crate::models::user::UserAuth;
use crate::models::DataPoolSqlite;
use crate::payloads::requests::{LoginRequest, RegisterRequest};
use crate::services::account::*;
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Result};
use log::error;

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

pub async fn register(
    req: web::Json<RegisterRequest>,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    match register_service(req.into_inner(), &pool).await {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            error!("unsuccessful user register message : {}", e);
            Ok(HttpResponse::BadRequest()
                .json(format!("Unsuccessful register {}", e)))
        }
    }
}

pub async fn login(
    req: web::Json<LoginRequest>,
    identity: Identity,
    pool: DataPoolSqlite,
) -> Result<HttpResponse> {
    match login_service(req.into_inner(), &pool).await {
        Ok(r) => {
            identity.remember(r.token.clone());
            Ok(HttpResponse::Ok().json(r))
        }
        Err(e) => {
            error!("login unsuccessful error message : {}", e);
            Ok(HttpResponse::Ok()
                .json("login unsuccessful"))
        }
    }
}

pub async fn logout(identity: Identity) -> Result<HttpResponse> {
    identity.forget();
    Ok(HttpResponse::Ok()
        .json("logout"))
}
