use crate::middlewares::authentication::get_user_id_from_request;
use crate::payloads::requests::UpdateAccessRequest;
use crate::services::access_service;
use actix_web::{web, HttpResponse};
use actix_web::{HttpRequest, Result};
use log::{error, info};
use sqlx::{Pool, SqliteConnection};

type PoolSqliteData = web::Data<Pool<SqliteConnection>>;

pub async fn add_access(
    pool: PoolSqliteData,
    req: HttpRequest,
    access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id_from_request(&pool.clone(), req).await;
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().content_type("application/json").body(e.to_string()));
    }
    let result = access_service::add_access(
        &pool,
        user_id.unwrap(),
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
    Ok(HttpResponse::Ok().content_type("application/json").body("nothing..."))
}

pub async fn remove_access(
    pool: PoolSqliteData,
    req: HttpRequest,
    access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id_from_request(&pool.clone(), req).await;
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().content_type("application/json").body(e.to_string()));
    }
    let result = access_service::delete_access(
        &pool,
        user_id.unwrap(),
        &*access_req.link,
        &*access_req.username,
    )
    .await;
    if let Ok(r) = result {
        info!("");
        return Ok(HttpResponse::Ok().content_type("application/json").json(r));
    }
    error!("");
    Ok(HttpResponse::Ok().content_type("application/json").body("nothing..."))
}
