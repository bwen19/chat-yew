use super::trigger::TriggerAction;
use crate::hooks::UseWebSocketHandle;

// ========================// RoomPageTrigger //======================== //

pub struct RoomPageTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for RoomPageTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl RoomPageTrigger {
    pub fn from_ws(ws: &UseWebSocketHandle) -> Self {
        Self {
            count: ws.trigger.count,
            action: ws.trigger.action.clone(),
        }
    }

    pub fn to_rooms(&self) -> RoomListTrigger {
        RoomListTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }

    pub fn to_messages(&self) -> MessageListTrigger {
        MessageListTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }

    pub fn to_members(&self) -> MemberListTrigger {
        MemberListTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }

    pub fn to_curr(&self) -> CurrRoomTrigger {
        CurrRoomTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }

    pub fn to_new(&self) -> NewRoomTrigger {
        NewRoomTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }
}

// ========================// RoomListTrigger //======================== //

#[derive(Clone)]
pub struct RoomListTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for RoomListTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Init | TriggerAction::Room(_) | TriggerAction::Message(_) => false,
                _ => true,
            }
    }
}

// ========================// MessageListTrigger //======================== //

#[derive(Clone)]
pub struct MessageListTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for MessageListTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Init | TriggerAction::Message(_) => false,
                _ => true,
            }
    }
}

// ========================// MemberListTrigger //======================== //

#[derive(Clone)]
pub struct MemberListTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for MemberListTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Init
                | TriggerAction::Member(_)
                | TriggerAction::Friend(_)
                | TriggerAction::NewFriend => false,
                _ => true,
            }
    }
}

// ========================// CurrRoomTrigger //======================== //

#[derive(Clone)]
pub struct CurrRoomTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for CurrRoomTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Room(ref aroom) => {
                    if aroom.curr_room == aroom.room_id {
                        false
                    } else {
                        true
                    }
                }
                TriggerAction::Init => false,
                _ => true,
            }
    }
}

// ========================// NewRoomTrigger //======================== //

#[derive(Clone)]
pub struct NewRoomTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for NewRoomTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Init | TriggerAction::Friend(_) | TriggerAction::NewFriend => false,
                _ => true,
            }
    }
}
