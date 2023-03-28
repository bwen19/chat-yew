use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ========================// Invitation //======================== //

#[derive(Deserialize, Serialize)]
pub struct Invitation {
    pub code: String,
    pub expire_at: DateTime<Utc>,
}

// ---------------- Create invitation ---------------- //
#[derive(Deserialize, Serialize, Validate)]
pub struct CreateInvitationRequest {
    #[validate(range(min = 4, message = "must be greater than 3"))]
    pub length: usize,
    #[validate(range(min = 1, message = "must be greater than 0"))]
    pub days: u64,
}

#[derive(Deserialize, Serialize)]
pub struct CreateInvitationResponse {
    pub invitation: Invitation,
}
