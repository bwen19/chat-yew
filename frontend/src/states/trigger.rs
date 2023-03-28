use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

// ========================// TriggerAction //======================== //

#[derive(Clone)]
pub struct AffectedRoom {
    pub room_id: i64,
    pub curr_room: i64,
}

#[derive(Clone)]
pub struct AffectedFriend {
    pub friend_id: i64,
    pub curr_friend: i64,
}

#[derive(Clone)]
pub enum TriggerAction {
    Init,
    Room(AffectedRoom),
    Message(AffectedRoom),
    Member(AffectedRoom),
    Friend(AffectedFriend),
    NewFriend,
}

// ========================// TriggerState //======================== //

pub struct TriggerState {
    pub count: u64,
    pub action: TriggerAction,
}

impl Default for TriggerState {
    fn default() -> Self {
        Self {
            count: 0,
            action: TriggerAction::Init,
        }
    }
}

impl Reducible for TriggerState {
    type Action = TriggerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let count = self.count + 1;
        Self { count, action }.into()
    }
}

pub type Trigger = UseReducerHandle<TriggerState>;
