use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========================// Friend //======================== //

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct FriendInfo {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    pub bio: String,
    pub status: String,
    pub room_id: i64,
    pub first: bool,
    pub create_at: DateTime<Utc>,
}

// ---------------- User's friends ---------------- //
#[derive(Deserialize, Serialize)]
pub struct UserFriendsResponse {
    pub friends: Vec<FriendInfo>,
}
// ---------------- Add friend ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct AddFriendRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub friend_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct AddFriendResponse {
    pub friend: FriendInfo,
}

// ---------------- Accept friend ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct AcceptFriendRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub friend_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct AcceptFriendResponse {
    pub friend: FriendInfo,
}

// ---------------- Refuse friend ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct RefuseFriendRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub friend_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct RefuseFriendResponse {
    pub friend_id: i64,
}

// ---------------- Delete friend ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteFriendRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub friend_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteFriendResponse {
    pub friend_id: i64,
}
