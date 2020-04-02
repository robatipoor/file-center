use crate::models::access::AccessType;
use crate::models::access_user::AccessUser;
use crate::models::file::File;
use crate::models::user::User;
use crate::payloads::responses::*;
use actix_web::web;
use log::{error, info};
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

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
    info!("");
    AccessUser::is_user_access(pool, user_id, file_id, access_id).await
}

pub async fn add_access(
    pool: &DataPoolSqlite,
    owner_id: i64,
    link: &str,
    username: &str,
    access: AccessType,
) -> anyhow::Result<ResponseBody<i64>> {
    let owner = is_owner(&pool, link, owner_id).await?;
    if owner {
        let file_id = File::find_id(&pool, link).await?;
        let user_id = User::find_id(&pool, username).await?;
        let exist = AccessUser::exist(&pool, user_id, file_id).await?;
        if !exist {
            let access_user = AccessUser::new(user_id, file_id, access as i64).await?;
            access_user.save(&pool).await?;
        } else {
            let access_user_id = AccessUser::find_id(&pool, user_id, file_id).await?;
            let row_affected =
                AccessUser::update_access(&pool, access_user_id, access as i64).await?;
            if row_affected == 1 {
                return Ok(ResponseBody::new(Status::SUCCESS, "Add Access"));
            }
        }
    }
    error!("");
    Err(anyhow!("Unsuccess"))
}

pub async fn delete_access(
    pool: &DataPoolSqlite,
    owner_id: i64,
    link: &str,
    username: &str,
) -> anyhow::Result<ResponseBody<String>> {
    let owner = is_owner(&pool, link, owner_id).await?;
    if owner {
        let file_id = File::find_id(&pool, link).await?;
        let user_id = User::find_id(&pool, username).await?;
        let result = AccessUser::delete(pool, user_id, file_id).await?;
        if result > 0 {
            return Ok(ResponseBody::new(Status::SUCCESS, "Delete Access"));
        }
    }
    error!("");
    Err(anyhow!("Delete UnSuccess!"))
}