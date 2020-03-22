use crate::config::constants::ONE_MONTH;
use crate::models::user::User;
use chrono::Utc;
use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub sub: String,
    pub rol: i32,
    pub iat: i64,
    pub exp: i64,
}

impl Token {
    pub fn new(user: User) -> Token {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        Token {
            sub: user.username,
            rol: user.role_id,
            iat: now,
            exp: now + ONE_MONTH,
        }
    }

    pub fn encode(&self) -> Result<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_ref()),
        )
    }

    pub fn decode(token: String) -> Result<TokenData<Self>> {
        // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
        decode::<Token>(
            &token,
            &DecodingKey::from_secret(std::env::var("SECRET_KEY").unwrap().as_ref()),
            &Validation::default(),
        )
    }
}
