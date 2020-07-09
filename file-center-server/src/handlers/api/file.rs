use crate::config::CONFIG;
use crate::models::file::File;
use crate::models::user::UserAuth;
use crate::models::DataPoolSqlite;
use crate::services::file::*;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use actix_web::{HttpRequest, Result};
use futures::StreamExt;
use log::error;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

pub async fn upload_file(
    pool: DataPoolSqlite,
    user_auth: UserAuth,
    mut payload: Multipart,
) -> Result<HttpResponse> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap().to_string();//TODO fix please
        let uuid = Uuid::new_v4().to_simple().to_string();
        let filepath =
            Path::new(CONFIG.path_file.as_str()).join(&*format!("{}-{}", uuid, filename));
        let conn = pool.clone();
        let file = File::new(&*filename, filepath.to_str().unwrap(), &*uuid, user_auth.id)
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

pub async fn list_file(pool: DataPoolSqlite, user_auth: UserAuth) -> Result<HttpResponse> {
    match list_files_service(&pool, user_auth.id).await {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => Ok(HttpResponse::Ok()
            .json(e.to_string())),
    }
}

pub async fn download_file(
    pool: DataPoolSqlite,
    user_auth: UserAuth,
    req: HttpRequest,
) -> Result<NamedFile, HttpResponse> {
    let link: String = match req.match_info().query("linkID").parse() {
        Ok(l) => l,
        Err(e) => {
            error!("message error : {}", e);
            return Err(HttpResponse::BadRequest()
                .content_type("application/json")
                .body("linkID not exist in request uri \n"));
        }
    };
    match download_file_service(&pool, user_auth, link).await {
        Ok(r) => Ok(r),
        Err(e) => Err(HttpResponse::Unauthorized()
            .json(e.to_string())),
    }
}

pub async fn manual_upload_file() -> Result<HttpResponse> {
    let response_body = r#" Manual Page
    # Upload File 
    curl -X POST --cookie 'RUSESSION=***' \
    -F file=@fileName \
    localhost:8080/api/file/upload
    # Get List File
    curl -X GET --cookie 'RUSESSION=***' \
    localhost:8080/api/file/list
    # Download File
    curl -X GET --cookie 'RUSESSION=***' \
    localhost:8080/api/file/download/linkID --output fileName
    # Add Access Read To Users
    curl -H "Content-Type: application/json" \
    -d '{"link":"linkID","username":"user-name","access_type":"Read"}' \
    --cookie 'RUSESSION=***' \
    -X POST http://localhost:8080/api/file/access
    # Remove Access 
    curl -H "Content-Type: application/json" \
    -d '{"link":"linkID","username":"user-name"}' \
    --cookie 'RUSESSION=***' \
    -X DELETE http://localhost:8080/api/file/access
    "#;
    Ok(HttpResponse::Ok()
        .json(response_body))
}
