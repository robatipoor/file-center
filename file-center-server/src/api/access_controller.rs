use crate::middlewares::authentication::get_user_id_from_request;
use crate::models::access::AccessType;
use crate::models::access_user::AccessUser;
use crate::models::file::File;
use crate::models::user::User;
use crate::payloads::requests::UpdateAccessRequest;
use crate::services::{access_service, file_service};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{http::StatusCode, web, Error, HttpResponse};
use actix_web::{HttpRequest, Result};
use futures::StreamExt;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;
type ResutResponse = Result<HttpResponse>;

pub async fn add_access(
    pool: PoolSqliteData,
    req: HttpRequest,
    update_access_req: web::Json<UpdateAccessRequest>,
) -> ResutResponse {
    let user_id = get_user_id_from_request(pool.clone(), req);
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().body(e));
    }
    access_service::is_owner(pool, &*update_access_req.link, user_id.unwrap());

    Ok(HttpResponse::Ok().body("nothing..."))
}

pub async fn remove_access(
    pool: PoolSqliteData,
    req: HttpRequest,
    update_access_req: web::Json<UpdateAccessRequest>,
) -> ResutResponse {
    let user_id = get_user_id_from_request(pool.clone(), req);
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().body(e));
    }
    access_service::is_owner(pool, &*update_access_req.link, user_id.unwrap());
    Ok(HttpResponse::Ok().body("nothing..."))
}
