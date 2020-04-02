use crate::middlewares::authentication::get_user_id_from_request;
use crate::payloads::requests::UpdateAccessRequest;
use crate::services::access_service;
use actix_web::{web, HttpResponse};
use actix_web::{HttpRequest, Result};
use sqlx::{Pool, SqliteConnection};

type PoolSqliteData = web::Data<Pool<SqliteConnection>>;

pub async fn add_access(
    pool: PoolSqliteData,
    req: HttpRequest,
    update_access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id_from_request(&pool.clone(), req).await;
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().body(e.to_string()));
    }
    access_service::is_owner(&pool, &*update_access_req.link, user_id.unwrap()).await;

    Ok(HttpResponse::Ok().body("nothing..."))
}

pub async fn remove_access(
    pool: PoolSqliteData,
    req: HttpRequest,
    update_access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id_from_request(&pool.clone(), req).await;
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().body(e.to_string()));
    }
    access_service::is_owner(&pool, &*update_access_req.link, user_id.unwrap()).await;
    Ok(HttpResponse::Ok().body("nothing..."))
}

pub async fn update_access(
    pool: PoolSqliteData,
    req: HttpRequest,
    update_access_req: web::Json<UpdateAccessRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id_from_request(&pool.clone(), req).await;
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().body(e.to_string()));
    }
    access_service::is_owner(&pool, &*update_access_req.link, user_id.unwrap()).await;
    Ok(HttpResponse::Ok().body("nothing..."))
}
