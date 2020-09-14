use crate::config::constants::{SESSION_EXPIRE_TIME_MINUTES, SESSION_KEY, SESSION_NAME};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use time::Duration;
pub mod access;
pub mod account;
pub mod file;

/// Gets the identidy service for injection into an Actix app
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(SESSION_KEY.as_bytes())
            .name(SESSION_NAME)
            .max_age_time(Duration::minutes(SESSION_EXPIRE_TIME_MINUTES))
            .secure(true),
    )
}
