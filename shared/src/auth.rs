use crate::user::UserInfo;
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========================// Auth //======================== //

// ---------------- Register ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 2,
        max = 50,
        message = "must be between 2 and 50 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 6,
        max = 50,
        message = "must be between 6 and 50 characters"
    ))]
    pub password: String,
    #[validate(length(
        min = 1,
        max = 50,
        message = "must be between 1 and 50 characters"
    ))]
    pub code: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub user: UserInfo,
}

// ---------------- Login ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(
        min = 2,
        max = 50,
        message = "must be between 2 and 50 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 6,
        max = 50,
        message = "must be between 6 and 50 characters"
    ))]
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub user: UserInfo,
    pub access_token: String,
}

// ---------------- Renew token ---------------- //
#[derive(Deserialize, Serialize)]
pub struct RenewTokenResponse {
    pub access_token: String,
}

// ---------------- Logout ---------------- //
#[derive(Deserialize, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}
