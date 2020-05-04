use crate::models::role::{Role, RoleName};
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
) -> anyhow::Result<ResponseBody<TokenResponse>> {
    let user_verified = User::verify(pool, login).await?;
    let token = Token::new(user_verified).encode()?;
    let token_response = TokenResponse::new(token.as_str());
    Ok(ResponseBody::new(Status::SUCCESS, "login").add_data(token_response))
}

pub async fn register_service(
    req: RegisterRequest,
    pool: &DataPoolSqlite,
) -> anyhow::Result<ResponseBody<String>> {
    let role = Role::new(RoleName::USER).await?;
    let user = User::new(&*req.username, &*req.password, &*req.email, role).await?;
    if !user.exist(pool).await? {
        let result_save = user.save(pool).await?;
        let response =
            ResponseBody::new(Status::SUCCESS, "User Register !").add_data(result_save.to_string());
        return Ok(response);
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
