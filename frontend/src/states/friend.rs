use super::trigger::TriggerAction;
use crate::hooks::UseWebSocketHandle;

// ========================// FriendPageTrigger //======================== //

pub struct FriendPageTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for FriendPageTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl FriendPageTrigger {
    pub fn from_ws(ws: &UseWebSocketHandle) -> Self {
        Self {
            count: ws.trigger.count,
            action: ws.trigger.action.clone(),
        }
    }

    pub fn to_curr(&self) -> CurrFriendTrigger {
        CurrFriendTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }

    pub fn to_new(&self) -> NewFriendTrigger {
        NewFriendTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }

    pub fn to_friends(&self) -> FriendListTrigger {
        FriendListTrigger {
            count: self.count,
            action: self.action.clone(),
        }
    }
}

// ========================// CurrFriendTrigger //======================== //

#[derive(Clone)]
pub struct CurrFriendTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for CurrFriendTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Friend(ref afriend) => {
                    if afriend.curr_friend == afriend.friend_id {
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

// ========================// NewFriendTrigger //======================== //

#[derive(Clone)]
pub struct NewFriendTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for NewFriendTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Init | TriggerAction::NewFriend | TriggerAction::Friend(_) => false,
                _ => true,
            }
    }
}

// ========================// FriendListTrigger //======================== //

#[derive(Clone)]
pub struct FriendListTrigger {
    count: u64,
    action: TriggerAction,
}

impl PartialEq for FriendListTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            || match other.action {
                TriggerAction::Init | TriggerAction::Friend(_) => false,
                _ => true,
            }
    }
}
