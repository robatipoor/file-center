use crate::models::user::UserAuth;
use crate::models::DataPoolSqlite;
use crate::payloads::requests::{RemoveAccessRequest, UpdateAccessRequest};
use crate::services::access::*;
use actix_web::Result;
use actix_web::{web, HttpResponse};
use log::{error, info};

pub async fn add_or_update_access(
    pool: DataPoolSqlite,
    user_auth: UserAuth,
    access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    match add_or_update_access_service(&pool, user_auth.id, &access_req.0).await {
        Ok(r) => {
            info!("update or add access");
            Ok(HttpResponse::Ok().json(r))
        }
        Err(e) => {
            error!("error {}", e);
            Ok(HttpResponse::Ok().json(e.to_string()))
        }
    }
}

pub async fn remove_access(
    pool: DataPoolSqlite,
    user_auth: UserAuth,
    access_req: web::Json<RemoveAccessRequest>,
) -> Result<HttpResponse> {
    match remove_access_service(&pool, user_auth.id, &access_req).await {
        Ok(r) => {
            info!("remove access");
            Ok(HttpResponse::Ok().json(r))
        }
        Err(e) => {
            error!("error {}", e);
            Ok(HttpResponse::Ok().json(e.to_string()))
        }
    }
}
