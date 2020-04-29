use crate::middlewares::authentication::get_user_id_from_request;
use crate::models::file::File;
use crate::services::file_service;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use actix_web::{HttpRequest, Result};
use futures::StreamExt;
use log::error;
use sqlx::{Pool, SqliteConnection};
use std::env;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

type PoolSqliteData = web::Data<Pool<SqliteConnection>>;

pub async fn upload_file(
    pool: PoolSqliteData,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = match get_user_id_from_request(&pool.clone(), req).await {
        Ok(id) => id,
        Err(e) => {
            error!("unautherized user message error : {}", e);
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json")
                .body("User not Autherized"));
        }
    };
    let path = match env::var("PATH_FILE") {
        Ok(p) => p,
        Err(e) => {
            error!("env PATH_FILE error : {}", e);
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("error !"));
        }
    };
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap().to_string();
        let uuid = Uuid::new_v4().to_simple().to_string();
        let filepath = Path::new(&path).join(&*format!("{}-{}", uuid, filename));
        let conn = pool.clone();
        let file = File::new(&*filename, filepath.to_str().unwrap(), &*uuid, user_id)
            .await
            .unwrap();
        file.save(&conn).await.unwrap();
        // File::create is blocking operation, use threadpool
        let mut f = web::block(move || std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

pub async fn list_file(pool: PoolSqliteData, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = match get_user_id_from_request(&pool.clone(), req).await {
        Ok(id) => id,
        Err(e) => {
            error!("unautherized user message error : {}", e);
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json")
                .body("User not Autherized \n"));
        }
    };
    let list = file_service::list_link_files(&pool, user_id).await;
    if let Err(e) = list {
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(e.to_string()));
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(list.unwrap()))
}

pub async fn download_file(
    pool: PoolSqliteData,
    req: HttpRequest,
) -> Result<NamedFile, HttpResponse> {
    let user_id = match get_user_id_from_request(&pool.clone(), req.clone()).await {
        Ok(id) => id,
        Err(e) => {
            error!("unautherized user message error : {}", e);
            return Err(HttpResponse::Unauthorized()
                .content_type("application/json")
                .body("User not Autherized \n"));
        }
    };

    let link: String = match req.match_info().query("linkID").parse() {
        Ok(l) => l,
        Err(e) => {
            error!("message error : {}", e);
            return Err(HttpResponse::BadRequest()
                .content_type("application/json")
                .body("linkID not exist in request uri \n"));
        }
    };

    let path = match file_service::download_path(&pool, &*link, user_id).await {
        Ok(list) => list,
        Err(e) => {
            error!("message error : {}", e);
            return Err(HttpResponse::NotFound()
                .content_type("application/json")
                .body("not found link id \n"));
        }
    };

    let named_file = match NamedFile::open(path) {
        Ok(n) => n,
        Err(e) => {
            error!("message error : {}", e);
            return Err(HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("failed open file"));
        }
    };

    Ok(named_file)
}

pub async fn manual_upload_file() -> Result<HttpResponse> {
    let text = r#"*** Post File Manual Page ***"#;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(text))
}
