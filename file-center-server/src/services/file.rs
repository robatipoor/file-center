use crate::models::access_user::AccessUser;
use crate::models::file::File;
use actix_web::web;
use log::{debug, error};
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

pub async fn get_id(pool: &DataPoolSqlite, link: &str) -> anyhow::Result<i64> {
    File::find_id(pool, link).await
}

pub async fn get_file_by_id(pool: &DataPoolSqlite, file_id: i64) -> anyhow::Result<File> {
    File::find_by_id(pool, file_id).await
}

pub async fn get_file_by_link(pool: &DataPoolSqlite, link: &str) -> anyhow::Result<File> {
    File::find_by_link(pool, link).await
}

pub async fn list_link_files(pool: &DataPoolSqlite, user_id: i64) -> anyhow::Result<Vec<String>> {
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
            return Err(anyhow!("file not exist error message : {}", e));
        }
    }
}

pub async fn save_file() {}

pub async fn update_file() {}

pub async fn delete_file() {}
