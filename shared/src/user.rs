use super::validator as VAL;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========================// User //======================== //

#[derive(Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    pub bio: String,
    pub role: String,
    pub deleted: bool,
    pub create_at: DateTime<Utc>,
}

// ---------------- Create user ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
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
    #[validate(custom = "VAL::validate_user_role")]
    pub role: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub user: UserInfo,
}

// ---------------- Delete users ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteUsersRequest {
    #[validate(custom = "VAL::validate_id_vec")]
    pub user_ids: Vec<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteUsersResponse {
    pub message: String,
}

// ---------------- Update user ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(range(min = 1, message = "user id is invalid"))]
    pub user_id: i64,
    #[validate(length(
        min = 2,
        max = 50,
        message = "must be between 2 and 50 characters"
    ))]
    pub username: Option<String>,
    #[validate(length(
        min = 6,
        max = 50,
        message = "must be between 6 and 50 characters"
    ))]
    pub password: Option<String>,
    #[validate(length(
        min = 2,
        max = 50,
        message = "must be between 2 and 50 characters"
    ))]
    pub nickname: Option<String>,
    #[validate(length(
        min = 1,
        max = 200,
        message = "must be between 1 and 200 characters"
    ))]
    pub avatar: Option<String>,
    #[validate(length(
        min = 1,
        max = 200,
        message = "must be between 1 and 200 characters"
    ))]
    pub bio: Option<String>,
    #[validate(custom = "VAL::validate_user_role")]
    pub role: Option<String>,
    pub deleted: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUserResponse {
    pub user: UserInfo,
}

// ---------------- List users ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct ListUsersRequest {
    #[validate(range(min = 1, message = "must be greater than 1"))]
    pub page_id: Option<i64>,
    #[validate(range(min = 5, max = 50, message = "must be between 5 and 50"))]
    pub page_size: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct ListUsersResponse {
    pub total: i64,
    pub users: Vec<UserInfo>,
}

// ---------------- Get user by name ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct GetUserByNameRequest {
    #[validate(length(
        min = 2,
        max = 50,
        message = "must be between 2 and 50 characters"
    ))]
    pub username: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct GetUserByNameResponse {
    pub user: Option<UserInfo>,
}
