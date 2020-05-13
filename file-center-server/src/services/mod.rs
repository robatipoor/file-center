use crate::config::constants::{SESSION_KEY, SESSION_NAME};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::web;
use sqlx::{Pool, SqliteConnection};
pub mod access;
pub mod account;
pub mod file;

pub type DataPoolSqlite = web::Data<Pool<SqliteConnection>>;

/// Gets the identidy service for injection into an Actix app
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(SESSION_KEY.as_bytes())
            .name(SESSION_NAME)
            .max_age_time(chrono::Duration::minutes(30))
            .secure(true),
    )
}
