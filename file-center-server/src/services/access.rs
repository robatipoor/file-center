use crate::models::access::AccessType;
use crate::models::access_user::AccessUser;
use crate::models::file::File;
use crate::models::user::User;
use actix_web::web;
use log::debug;
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;
const WRITE_ACCESS_ID: i64 = AccessType::Write as i64;
const READ_ACCESS_ID: i64 = AccessType::Read as i64;

pub async fn is_owner(pool: &DataPoolSqlite, link: &str, user_id: i64) -> anyhow::Result<bool> {
    File::is_owner(pool, link, user_id).await
}

pub async fn is_read_access(
    pool: &DataPoolSqlite,
    file_id: i64,
    user_id: i64,
) -> anyhow::Result<bool> {
    debug!(
        "check user id {} is read access to file id {}",
        user_id, file_id
    );
    AccessUser::is_user_access(pool, user_id, file_id, READ_ACCESS_ID).await
}

pub async fn is_write_access(
    pool: &DataPoolSqlite,
    file_id: i64,
    user_id: i64,
) -> anyhow::Result<bool> {
    debug!(
        "check user id {} is write access to file id {}",
        user_id, file_id
    );
    AccessUser::is_user_access(pool, user_id, file_id, WRITE_ACCESS_ID).await
}

pub async fn add_or_update_access_service(
    pool: &DataPoolSqlite,
    owner_id: i64,
    link: &str,
    username: &str,
    access: AccessType,
) -> anyhow::Result<String> {
    match AccessUser::find_id(&pool, username, link).await {
        Ok(access_user_id) => {
            let row_affected =
                AccessUser::update_access(&pool, access_user_id, access as i64).await?;
            if row_affected == 1 {
                return Ok("Update Access".to_string());
            } else {
                return Err(anyhow!("Unsuccessfull Update Access"));
            }
        }
        Err(_) => {
            if is_owner(&pool, link, owner_id).await? {
                let file_id = File::find_id(&pool, link).await?;
                let user_id = User::find_id(&pool, username).await?;
                let access_user = AccessUser::new(user_id, file_id, access as i64).await?;
                let _id = access_user.save(&pool).await?;
                return Ok("Add Access".to_string());
            } else {
                return Err(anyhow!("User Not Owner"));
            }
        }
    }
}

pub async fn remove_access_service(
    pool: &DataPoolSqlite,
    owner_id: i64,
    link: &str,
    username: &str,
) -> anyhow::Result<String> {
    debug!(
        "delete access owner_id {} link {} username {}",
        owner_id, link, username
    );
    if is_owner(&pool, link, owner_id).await? {
        let file_id = File::find_id(&pool, link).await?;
        let user_id = User::find_id(&pool, username).await?;
        if AccessUser::delete(pool, user_id, file_id).await? > 0 {
            return Ok("Delete Access Successfully".to_string());
        } else {
            return Err(anyhow!("Delete Unsuccessful"));
        }
    }
    Err(anyhow!("User Not Owner"))
}
