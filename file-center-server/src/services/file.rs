use crate::models::access_user::AccessUser;
use crate::models::file::File;
use crate::models::user::UserAuth;
use crate::models::DataPoolSqlite;
use crate::payloads::responses::FileResponse;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use log::{debug, error};

pub async fn uplaod_file_service<T: AsRef<str>>(
    _pool: &DataPoolSqlite,
    _user_auth: UserAuth,
    mut _payload: Multipart,
) {
    todo!()
}

pub async fn download_file_service<T: AsRef<str>>(
    pool: &DataPoolSqlite,
    user_auth: UserAuth,
    link: T,
) -> anyhow::Result<NamedFile> {
    if let Err(e) = user_access_to_link(&pool, link.as_ref(), user_auth.id).await {
        error!("Unauthorized : {}", e);
        return Err(anyhow!("user not access to file \n"));
    };

    let path: String = match get_download_path(&pool, link.as_ref()).await {
        Ok(list) => list,
        Err(e) => {
            error!("message error : {}", e);
            return Err(anyhow!("not found link id \n"));
        }
    };

    let named_file = match NamedFile::open(path) {
        Ok(n) => n,
        Err(e) => {
            error!("message error : {}", e);
            return Err(anyhow!("failed open file"));
        }
    };
    Ok(named_file)
}

pub async fn get_id(pool: &DataPoolSqlite, link: &str) -> anyhow::Result<i64> {
    File::find_id(pool, link).await
}

pub async fn get_file_by_id(pool: &DataPoolSqlite, file_id: i64) -> anyhow::Result<File> {
    File::find_by_id(pool, file_id).await
}

pub async fn get_file_by_link(pool: &DataPoolSqlite, link: &str) -> anyhow::Result<File> {
    File::find_by_link(pool, link).await
}

pub async fn list_files_service(
    pool: &DataPoolSqlite,
    user_id: i64,
) -> anyhow::Result<Vec<FileResponse>> {
    File::find_all_link_files(pool, user_id).await
}

pub async fn user_access_to_link(
    pool: &DataPoolSqlite,
    link: &str,
    user_id: i64,
) -> anyhow::Result<()> {
    if File::is_owner(pool, link, user_id).await? {
        debug!("user id {} is owner link file {}", user_id, link);
        Ok(())
    } else if AccessUser::user_has_access_by_link(pool, link, user_id).await? {
        debug!("user id {} is has access link file {}", user_id, link);
        Ok(())
    } else {
        error!("user not access ");
        Err(anyhow!("user not access "))
    }
}

pub async fn get_download_path(pool: &DataPoolSqlite, link: &str) -> anyhow::Result<String> {
    match File::find_path_by_link(pool, link).await {
        Ok(path) => Ok(path),
        Err(e) => {
            error!("file not exist error message {}", e);
            Err(anyhow!("file not exist error message : {}", e))
        }
    }
}

pub async fn save_file() {}

pub async fn update_file() {}

pub async fn delete_file() {}
