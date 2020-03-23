use crate::middlewares::authentication::get_claims_from_request;
use crate::models::file::File;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{http::StatusCode, web, Error, HttpResponse};
use actix_web::{HttpRequest, Result};
use futures::StreamExt;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::env;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;

pub async fn upload_file(
    mut payload: Multipart,
    req: HttpRequest,
    pool: PoolSqliteData,
) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    let claims = get_claims_from_request(req);
    if claims.is_some() {
        while let Some(item) = payload.next().await {
            let mut field = item?;
            let content_type = field.content_disposition().unwrap();
            let filename = content_type.get_filename().unwrap().to_string();
            let path = env::var("PATH_FILE").unwrap();
            let filepath = Path::new(&path).join(&*filename);
            let uuid = "";
            let conn = pool.clone().get().unwrap();
            // File::create is blocking operation, use threadpool
            let mut f = web::block(move || {
                File::new(&*filename, filepath.to_str().unwrap(), uuid, 3)
                    .save(&conn)
                    .expect("save file failed");
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
    } else {
        Err(HttpResponse::build(StatusCode::UNAUTHORIZED).into())
    }
}

pub async fn download_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn help_upload_file() -> HttpResponse {
    let text = r#""#;
    HttpResponse::Ok().body(text)
}
