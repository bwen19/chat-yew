use super::validator as VAL;
use crate::{friend::FriendInfo, room::RoomInfo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========================// Message //======================== //

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct MessageInfo {
    pub id: i64,
    pub sid: i64,
    pub name: String,
    pub avatar: String,
    pub content: String,
    pub kind: String,
    pub send_at: DateTime<Utc>,
}

// ---------------- Init ---------------- //
/// Used to pass initial rooms and friends
#[derive(Deserialize, Serialize)]
pub struct InitialResponse {
    pub rooms: Vec<RoomInfo>,
    pub friends: Vec<FriendInfo>,
}

// ---------------- New message ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct NewMessageRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub room_id: i64,
    #[validate(length(min = 1, max = 500, message = "must be between 1 and 500 characters"))]
    pub content: String,
    #[validate(custom = "VAL::validate_message_kind")]
    pub kind: String,
}

/// Used to pass a single message to client
#[derive(Deserialize, Serialize)]
pub struct NewMessageResponse {
    pub room_id: i64,
    pub message: MessageInfo,
}
