use crate::{member::MemberInfo, message::MessageInfo, validator as VAL};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========================// Room //======================== //

#[derive(Deserialize, Serialize)]
pub struct RoomInfo {
    pub id: i64,
    pub name: String,
    pub cover: String,
    pub category: String,
    pub create_at: DateTime<Utc>,
    pub members: Vec<MemberInfo>,
    pub messages: Vec<MessageInfo>,
}

// ---------------- User's rooms ---------------- //
#[derive(Deserialize, Serialize)]
pub struct UserRoomsResponse {
    pub rooms: Vec<RoomInfo>,
}

// ---------------- New room ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct NewRoomRequest {
    #[validate(length(min = 2, max = 50, message = "must be between 2 and 50 characters"))]
    pub name: String,
    #[validate(
        length(min = 3, message = "must have at least 3 members"),
        custom = "VAL::validate_id_vec"
    )]
    pub member_ids: Vec<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct NewRoomResponse {
    pub room: RoomInfo,
}

// ---------------- Delete room ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteRoomRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub room_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteRoomResponse {
    pub room_id: i64,
}

// ---------------- Leave room ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct LeaveRoomRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub room_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct LeaveRoomResponse {
    pub room_id: i64,
}

// ---------------- New room name ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct NewRoomNameResquest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub room_id: i64,
    #[validate(length(min = 2, max = 50, message = "must be between 2 and 50 characters"))]
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewRoomNameResponse {
    pub room_id: i64,
    pub name: String,
}
