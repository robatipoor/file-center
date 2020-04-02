use crate::config::constants;
use crate::models::access_user::AccessUser;
use crate::models::file::File;
use crate::models::role::{Role, RoleName};
use crate::models::user::User;
use crate::payloads::requests::*;
use crate::payloads::responses::*;
use crate::utils::jwt::Token;
use actix_web::{http::StatusCode, web};
use actix_web::{Error, Result};
use log::{error, info};
use serde_json::json;
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

pub async fn download_path(
    pool: &DataPoolSqlite,
    link: &str,
    user_id: i64,
) -> anyhow::Result<String> {
    let is_owner = File::is_owner(pool, link, user_id).await?;
    if is_owner {
        return File::find_path_by_link(pool, link).await;
    } else {
        let access = AccessUser::user_has_access_by_link(pool, link, user_id).await?;
        if access {
            return File::find_path_by_link(pool, link).await;
        }
    }
    Err(anyhow!("user not access "))
}

pub async fn save_file() {}

pub async fn update_file() {}

pub async fn delete_file() {}
