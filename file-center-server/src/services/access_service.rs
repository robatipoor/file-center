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
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;
type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;
type ResultResponse = Result<TokenBodyResponse, ServiceError>;

pub fn is_owner(pool: PoolSqliteData, link:&str, user_id: i32) -> Result<bool, String> {
    File::is_owner(&pool.get().unwrap(), link, user_id).map_err(|e| format!("{:?}", e))
}

pub fn is_read_access(pool: PoolSqliteData, file_id: i32, user_id: i32) -> Result<bool, String> {
    let access_id = AccessType::Read as i32;
    AccessUser::is_user_access(&pool.get().unwrap(), user_id, file_id, access_id)
        .map_err(|e| format!("{:?}", e))
}

pub fn is_write_access(pool: PoolSqliteData, file_id: i32, user_id: i32) -> Result<bool, String> {
    let access_id = AccessType::Write as i32;
    AccessUser::is_user_access(&pool.get().unwrap(), user_id, file_id, access_id)
        .map_err(|e| format!("{:?}", e))
}

pub fn add_access(pool: PoolSqliteData, access_user: AccessUser) -> Result<usize, String> {
    access_user
        .save(&pool.get().unwrap())
        .map_err(|e| format!("{:?}", e))
}

pub fn update_access(
    pool: PoolSqliteData,
    access_user_id: i32,
    access_id: i32,
) -> Result<usize, String> {
    AccessUser::update_access(&pool.get().unwrap(), access_user_id, access_id)
        .map_err(|e| format!("{:?}", e))
}

pub fn delete_access(pool: PoolSqliteData, file_id: i32, user_id: i32) {
    todo!()
}
