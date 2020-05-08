use crate::models::user::UserAuth;
use crate::payloads::requests::UpdateAccessRequest;
use crate::services::access::*;
use actix_web::Result;
use actix_web::{web, HttpResponse};
use log::{error, info};
use sqlx::{Pool, SqliteConnection};

type PoolSqliteData = web::Data<Pool<SqliteConnection>>;

pub async fn add_access(
    pool: PoolSqliteData,
    user_auth: UserAuth,
    access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    let result = add_or_update_access_service(
        &pool,
        user_auth.id,
        &*access_req.link,
        &*access_req.username,
        access_req.access_type,
    )
    .await;
    if let Ok(b) = result {
        info!("");
        return Ok(HttpResponse::Ok().content_type("application/json").json(b));
    }
    error!("");
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("nothing..."))
}

pub async fn remove_access(
    pool: PoolSqliteData,
    user_auth: UserAuth,
    access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    let result = remove_access_service(
        &pool,
        user_auth.id,
        &*access_req.link,
        &*access_req.username,
    )
    .await;
    if let Ok(r) = result {
        info!("");
        return Ok(HttpResponse::Ok().content_type("application/json").json(r));
    }
    error!("");
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("nothing..."))
}
