use crate::config::constants::TOKEN_EXPIRE_TIME;
use crate::config::CONFIG;
use crate::models::user::User;
use chrono::Utc;

use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub sub: i64,
    pub rol: i64,
    pub iat: i64,
    pub exp: i64,
}

impl Token {
    pub fn new(user: User) -> Token {
        let now = Utc::now().timestamp_millis() / 1000; // milisecond to second
        Token {
            sub: user.id,
            rol: user.role_id,
            iat: now,
            exp: now + TOKEN_EXPIRE_TIME,
        }
    }

    pub fn encode(&self) -> Result<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(CONFIG.secret_key.as_ref()),
        )
    }

    pub fn decode(token: String) -> Result<TokenData<Self>> {
        // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
        decode::<Token>(
            &token,
            &DecodingKey::from_secret(CONFIG.secret_key.as_ref()),
            &Validation::default(),
        )
    }
}
