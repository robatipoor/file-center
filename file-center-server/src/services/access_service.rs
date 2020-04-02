use crate::config::constants;
use crate::models::access::AccessType;
use crate::models::access_user::AccessUser;
use crate::models::file::File;
use crate::models::role::{Role, RoleName};
use crate::models::user::User;
use crate::payloads::requests::*;
use crate::payloads::responses::*;
use crate::services::file_service;
use crate::utils::jwt::Token;
use actix_web::Result;
use actix_web::{http::StatusCode, web};
use log::{error, info};
use serde_json::json;
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;
type ResultResponse = Result<TokenBodyResponse, ServiceError>;

pub async fn is_owner(pool: &DataPoolSqlite, link: &str, user_id: i64) -> anyhow::Result<bool> {
    File::is_owner(pool, link, user_id).await
}

pub async fn is_read_access(
    pool: &DataPoolSqlite,
    file_id: i64,
    user_id: i64,
) -> anyhow::Result<bool> {
    let access_id = AccessType::Read as i64;
    AccessUser::is_user_access(pool, user_id, file_id, access_id).await
}

pub async fn is_write_access(
    pool: &DataPoolSqlite,
    file_id: i64,
    user_id: i64,
) -> anyhow::Result<bool> {
    let access_id = AccessType::Write as i64;
    AccessUser::is_user_access(pool, user_id, file_id, access_id).await
}

pub async fn add_access(pool: &DataPoolSqlite, access_user: AccessUser) -> anyhow::Result<i64> {
    access_user.save(pool).await
}

pub async fn update_access(
    pool: &DataPoolSqlite,
    access_user_id: i64,
    access_id: i64,
) -> anyhow::Result<u64> {
    AccessUser::update_access(pool, access_user_id, access_id).await
}

pub async fn delete_access(
    pool: &DataPoolSqlite,
    file_id: i64,
    user_id: i64,
) -> anyhow::Result<()> {
    todo!()
}
