use crate::config::constants;
use crate::models::role::{Role, RoleName};
use crate::models::user::User;
use crate::payloads::requests::*;
use crate::payloads::responses::*;
use crate::utils::jwt::Token;
use actix_web::{http::StatusCode, web};
use log::{error, info};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;
type PoolSqliteData = web::Data<Pool<SqliteConnectionManager>>;
type ResultResponse = actix_web::Result<TokenBodyResponse, ServiceError>;

pub fn login(login: LoginRequest, pool: PoolSqliteData) -> ResultResponse {
    match User::verify(&pool.get().unwrap(), login) {
        Ok(logged_user) => {
            match serde_json::from_value(
                json!({ "token": Token::new(logged_user).encode().unwrap(), "token_type": "bearer" }),
            ) {
                Ok(token_res) => Ok(token_res),
                Err(e) => {
                    error!("{}", e);
                    Err(ServiceError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
                    ))
                }
            }
        }

        Err(e) => {
            error!("{}", e);
            Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_LOGIN_FAILED.to_string(),
            ))
        }
    }
}

pub fn register(
    req: RegisterRequest,
    pool: PoolSqliteData,
) -> Result<ResponseBody<String>, ServiceError> {
    let role = Role::new(RoleName::RoleUser);
    let user = User::new(&*req.username, &*req.password, &*req.email, role);
    let result = user.exist(&pool.get().unwrap());
    if result.is_ok() && result.unwrap() {
        Err(ServiceError::new(
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            "User Exist !".to_owned(),
        ))
    } else {
        let result_save = user.save(&pool.get().unwrap());
        if result_save.is_ok() {
            Ok(ResponseBody::new(
                true,
                "Success Register !".to_owned(),
                Some(result_save.unwrap().to_string()),
            ))
        } else {
            Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Sorry Register Failed !".to_owned(),
            ))
        }
    }
}

pub fn get_list_users(
    _req: RegisterRequest,
    _pool: PoolSqliteData,
) -> Result<ResponseBody<String>, ServiceError> {
    todo!()
}

pub fn add_role_admin(
    _req: RegisterRequest,
    _pool: PoolSqliteData,
) -> Result<ResponseBody<String>, ServiceError> {
    todo!()
}

pub fn delete_account(
    _req: RegisterRequest,
    _pool: PoolSqliteData,
) -> Result<ResponseBody<String>, ServiceError> {
    todo!()
}

pub fn update_account(
    _req: UpdateAccoutRequest,
    _pool: PoolSqliteData,
) -> Result<ResponseBody<String>, ServiceError> {
    todo!()
}
