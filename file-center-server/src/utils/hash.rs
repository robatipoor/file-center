use crate::config::constants::BCRYPT_COST;

#[derive(Debug)]
pub struct Bcrypt;

impl Bcrypt {
    pub fn hash(plain_password: &str) -> String {
        bcrypt::hash(plain_password, BCRYPT_COST).expect("hash password failed!")
    }

    pub fn verify(plain_password: &str, hash: &str) -> bool {
        bcrypt::verify(plain_password.as_bytes(), hash).expect("verify password failed!")
    }
}
