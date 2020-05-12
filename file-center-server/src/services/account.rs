use crate::models::role::RoleName;
use crate::models::user::User;
use crate::payloads::requests::*;
use crate::payloads::responses::*;
use crate::utils::jwt::Token;
use actix_web::web;
use sqlx::{Pool, SqliteConnection};

type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

pub async fn login_service(
    login: LoginRequest,
    pool: &DataPoolSqlite,
) -> anyhow::Result<TokenResponse> {
    let user_verified = User::verify(pool, login).await?;
    let token = Token::new(user_verified).encode()?;
    Ok(TokenResponse::new(token.as_str()))
}

pub async fn register_service(
    req: RegisterRequest,
    pool: &DataPoolSqlite,
) -> anyhow::Result<String> {
    let user_role = RoleName::USER;
    let user = User::new(&*req.username, &*req.password, &*req.email, user_role).await?;
    if !user.exist(pool).await? {
        let _result_save = user.save(pool).await?;
        return Ok("User Success Register !".to_string());
    }
    Err(anyhow!("User Exist !"))
}

pub async fn get_list_users(
    _req: RegisterRequest,
    _pool: &DataPoolSqlite,
) -> anyhow::Result<ResponseBody<String>> {
    todo!()
}

pub async fn add_role_admin(
    _req: RegisterRequest,
    _pool: &DataPoolSqlite,
) -> anyhow::Result<ResponseBody<String>> {
    todo!()
}

pub async fn delete_account(
    _req: RegisterRequest,
    _pool: &DataPoolSqlite,
) -> anyhow::Result<ResponseBody<String>> {
    todo!()
}

pub async fn update_account(
    _req: UpdateAccoutRequest,
    _pool: &DataPoolSqlite,
) -> anyhow::Result<ResponseBody<String>> {
    todo!()
}
