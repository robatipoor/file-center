use crate::config::constants;
use crate::models::file::File;
use crate::models::role::{Role, RoleName};
use crate::models::user::User;
use crate::payloads::requests::*;
use crate::payloads::responses::*;
use crate::utils::jwt::Token;
use actix_web::{http::StatusCode, web};
use actix_web::{Error, Result};
use log::{error, info};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;
type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;

pub fn get_id(pool: PoolSqliteData, link: &str) -> Result<i32, String> {
    File::find_id(&pool.get().unwrap(), link).map_err(|e| format!("{:?}", e))
}

pub fn get_file_by_id(pool: PoolSqliteData, file_id: i32) -> Result<File, String> {
    File::find_by_id(&pool.get().unwrap(), file_id).map_err(|e| format!("{:?}", e))
}

pub fn get_file_by_link(pool: PoolSqliteData, link: &str) -> Result<File, String> {
    File::find_by_link(&pool.get().unwrap(), link).map_err(|e| format!("{:?}", e))
}

pub fn save_file() {}

pub fn update_file() {}

pub fn delete_file() {}
