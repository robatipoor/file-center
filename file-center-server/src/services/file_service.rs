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
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;
type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;

pub fn get_id(pool: PoolSqliteData, link: &str) -> Result<i32, String> {
    File::find_id(&pool.get().unwrap(), link).map_err(|e| e.to_string())
}

pub fn get_file_by_id(pool: PoolSqliteData, file_id: i32) -> Result<File, String> {
    File::find_by_id(&pool.get().unwrap(), file_id).map_err(|e| e.to_string())
}

pub fn get_file_by_link(pool: PoolSqliteData, link: &str) -> Result<File, String> {
    File::find_by_link(&pool.get().unwrap(), link).map_err(|e| e.to_string())
}

pub fn list_link_files(pool: PoolSqliteData, user_id: i32) -> Result<Vec<String>, String> {
    File::find_all_link_files(&pool.get().unwrap(), user_id).map_err(|e| e.to_string())
}

pub fn download_path(pool: PoolSqliteData, link: &str, user_id: i32) -> Result<String, String> {
    let is_owner = File::is_owner(&pool.get().unwrap(), link, user_id);
    if is_owner.is_ok() && is_owner.unwrap() {
        return File::find_path_by_link(&pool.get().unwrap(), link).map_err(|e| e.to_string());
    } else {
        let access = AccessUser::user_has_read_access(&pool.get().unwrap(), link, user_id);
        if access.is_ok() && access.unwrap() {
            return File::find_path_by_link(&pool.get().unwrap(), link).map_err(|e| e.to_string());
        }
    }
    Err("user not access ".to_owned())
}

pub fn save_file() {}

pub fn update_file() {}

pub fn delete_file() {}
