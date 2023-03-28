use crate::validator as VAL;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========================// Room Members //======================== //

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct MemberInfo {
    pub id: i64,
    pub name: String,
    pub avatar: String,
    pub rank: String,
    pub join_at: DateTime<Utc>,
}

// ---------------- Add members ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct AddMembersRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub room_id: i64,
    #[validate(
        length(min = 1, message = "must have at least 1 members"),
        custom = "VAL::validate_id_vec"
    )]
    pub member_ids: Vec<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct AddMembersResponse {
    pub room_id: i64,
    pub members: Vec<MemberInfo>,
}

// ---------------- Delete members ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteMembersRequest {
    #[validate(range(min = 1, message = "invalid ID"))]
    pub room_id: i64,
    #[validate(
        length(min = 1, message = "must have at least 1 members"),
        custom = "VAL::validate_id_vec"
    )]
    pub member_ids: Vec<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteMembersResponse {
    pub room_id: i64,
    pub member_ids: Vec<i64>,
}
