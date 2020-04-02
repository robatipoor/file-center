use crate::middlewares::authentication::get_user_id_from_request;
use crate::models::file::File;
use crate::services::file_service;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use actix_web::{HttpRequest, Result};
use futures::StreamExt;
use sqlx::{Pool, SqliteConnection};
use std::env;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

type PoolSqliteData = web::Data<Pool<SqliteConnection>>;
type ResutResponse = Result<HttpResponse>;

pub async fn upload_file(
    pool: PoolSqliteData,
    mut payload: Multipart,
    req: HttpRequest,
) -> ResutResponse {
    // iterate over multipart stream
    let user_id = get_user_id_from_request(&pool.clone(), req).await.unwrap();
    let path = env::var("PATH_FILE").unwrap();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap().to_string();
        let uuid = Uuid::new_v4().to_string().replace("-", "");
        let filepath = Path::new(&path).join(&*format!("{}-{}", uuid, filename));
        let conn = pool.clone();
        let file = File::new(&*filename, filepath.to_str().unwrap(), &*uuid, user_id)
            .await
            .unwrap();
        file.save(&conn);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(move || {
            return std::fs::File::create(filepath);
        })
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

pub async fn list_file(pool: PoolSqliteData, req: HttpRequest) -> ResutResponse {
    let user_id = get_user_id_from_request(&pool.clone(), req).await;
    if let Err(e) = user_id {
        return Ok(HttpResponse::Ok().body(e.to_string()));
    }
    let list = file_service::list_link_files(&pool, user_id.unwrap()).await;
    if let Err(e) = list {
        return Ok(HttpResponse::Ok().body(e.to_string()));
    }
    Ok(HttpResponse::Ok().json(list.unwrap()))
}

pub async fn download_file(pool: PoolSqliteData, req: HttpRequest) -> Result<NamedFile> {
    let link: String = req.match_info().query("linkID").parse().unwrap();
    let user_id = get_user_id_from_request(&pool.clone(), req).await.unwrap();
    let path = file_service::download_path(&pool, &*link, user_id)
        .await
        .unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn help_upload_file() -> ResutResponse {
    let text = r#" Post File "#;
    Ok(HttpResponse::Ok().body(text))
}
