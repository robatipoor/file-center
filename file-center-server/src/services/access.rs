use crate::models::access::AccessType;
use crate::models::access_user::AccessUser;
use crate::models::file::File;
use crate::models::user::User;
use crate::payloads::requests::{RemoveAccessRequest, UpdateAccessRequest};
use actix_web::web;
use log::{debug, error, info};
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;
const WRITE_ACCESS_ID: i64 = AccessType::Write as i64;
const READ_ACCESS_ID: i64 = AccessType::Read as i64;

pub async fn is_owner<T: AsRef<str>>(
    pool: &DataPoolSqlite,
    link: T,
    user_id: i64,
) -> anyhow::Result<bool> {
    info!("link {} user id {}", link.as_ref(), user_id);
    File::is_owner(pool, link.as_ref(), user_id).await
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
    req: &UpdateAccessRequest,
) -> anyhow::Result<String> {
    // update access if exist no need check is_owner
    match AccessUser::find_id(&pool, req.username.as_str(), req.link.as_str()).await {
        Ok(access_user_id) => {
            let row_affected =
                AccessUser::update_access(&pool, access_user_id, req.access_type as i64).await?;
            if row_affected == 1 {
                Ok("update access".to_string())
            } else {
                Err(anyhow!("unsuccessfull update access"))
            }
        }
        Err(e) => {
            error!("error {} ", e);
            if is_owner(&pool, req.link.as_str(), owner_id).await? {
                info!("{} user is owner !", owner_id);
                let file_id = File::find_id(&pool, req.link.as_str()).await?;
                let user_id = User::find_id(&pool, req.username.as_str()).await?;
                let access_user = AccessUser::new(user_id, file_id, req.access_type as i64).await?;
                let _id = access_user.save(&pool).await?;
                Ok("Add Access".to_string())
            } else {
                Err(anyhow!("User Not Owner"))
            }
        }
    }
}

pub async fn remove_access_service(
    pool: &DataPoolSqlite,
    owner_id: i64,
    access_req: &RemoveAccessRequest,
) -> anyhow::Result<String> {
    debug!(
        "delete access owner_id {} link {} username {}",
        owner_id, access_req.link, access_req.username
    );
    if is_owner(&pool, access_req.link.as_str(), owner_id).await? {
        let file_id = File::find_id(&pool, access_req.link.as_str()).await?;
        let user_id = User::find_id(&pool, access_req.username.as_str()).await?;
        if AccessUser::delete(pool, user_id, file_id).await? > 0 {
            return Ok("Delete Access Successfully".to_string());
        } else {
            return Err(anyhow!("Delete Unsuccessful"));
        }
    }
    Err(anyhow!("User Not Owner"))
}
