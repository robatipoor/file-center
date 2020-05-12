use crate::models::access::AccessType;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccoutRequest {
    pub new_username: Option<String>,
    pub new_password: Option<String>,
    pub new_email: Option<String>,
    pub old_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccessRequest {
    pub link: String,
    pub username: String,
    pub access_type: AccessType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveAccessRequest {
    pub link: String,
    pub username: String,
}
