use super::{
    types::{
        time_ago, ConvertToMessage, CurrRoomItem, FriendAttr, FriendItem, MemberItem,
        MessageContent, MessageItem, PersonalRoom, PublicRoom, Relation, RoomItem,
    },
    websocket::WebSocketState,
};
use crate::hooks::UseWebSocketHandle;
use chrono::Local;
use futures::SinkExt;
use gloo_net::websocket::Message;
use shared::{
    event::ClientEvent,
    friend::{AcceptFriendRequest, AddFriendRequest, DeleteFriendRequest, RefuseFriendRequest},
    member::{AddMembersRequest, DeleteMembersRequest},
    message::NewMessageRequest,
    room::{DeleteRoomRequest, LeaveRoomRequest, NewRoomNameResquest, NewRoomRequest},
    user::GetUserByNameResponse,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use yew::AttrValue;

// ========================// ChatState //======================== //

pub struct ChatState {
    inner: Rc<RefCell<WebSocketState>>,
}

impl PartialEq for ChatState {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Clone for ChatState {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl ChatState {
    pub fn from_ws(ws: &UseWebSocketHandle) -> Self {
        Self {
            inner: ws.inner.clone(),
        }
    }

    pub fn curr_room(&self) -> i64 {
        self.inner.borrow().curr_room
    }

    pub fn set_curr_room(&self, room_id: i64) {
        self.inner.borrow_mut().curr_room = room_id;
        if let Some(room) = self
            .inner
            .borrow_mut()
            .rooms
            .iter_mut()
            .find(|r| r.id == room_id)
        {
            room.unreads = 0;
        }
    }

    pub fn curr_friend(&self) -> i64 {
        self.inner.borrow().curr_friend
    }

    pub fn set_curr_friend(&self, friend_id: i64) {
        self.inner.borrow_mut().curr_friend = friend_id;
    }

    pub fn get_curr_room(&self, room_id: i64) -> Option<CurrRoomItem> {
        if let Some(room) = self.inner.borrow().rooms.iter().find(|r| r.id == room_id) {
            let curr_user = self.inner.borrow().curr_user;

            match room.category.as_str() {
                "public" => Some(CurrRoomItem::Public(PublicRoom::from_attr(room, curr_user))),
                "private" => {
                    if let Some(member) = room.members.iter().find(|x| x.id != curr_user) {
                        if let Some(friend) = self
                            .inner
                            .borrow()
                            .friends
                            .iter()
                            .find(|x| x.id == member.id)
                        {
                            Some(CurrRoomItem::Private(friend.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "personal" => Some(CurrRoomItem::Personal(PersonalRoom::from_attr(room))),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_rank(&self, room_id: i64) -> AttrValue {
        if let Some(room) = self.inner.borrow().rooms.iter().find(|x| x.id == room_id) {
            let curr_user = self.inner.borrow().curr_user;
            room.members
                .iter()
                .find(|x| x.id == curr_user)
                .map(|x| x.rank.clone().into())
                .unwrap_or(AttrValue::from("member"))
        } else {
            AttrValue::from("member")
        }
    }

    pub fn get_rooms(&self) -> Vec<RoomItem> {
        let mut rooms = Vec::new();
        let curr_user = self.inner.borrow().curr_user;

        for room in self.inner.borrow().rooms.iter().rev() {
            rooms.push(RoomItem::from_attr(room, curr_user));
        }
        rooms
    }

    pub fn search_rooms(&self, target: &String) -> Vec<RoomItem> {
        let mut rooms = Vec::new();
        let curr_user = self.inner.borrow().curr_user;

        for room in self.inner.borrow().rooms.iter().rev() {
            let room_item = RoomItem::from_attr(room, curr_user);
            if room_item.name.contains(target) {
                rooms.push(room_item);
            }
        }
        rooms
    }

    pub fn get_messages(&self, room_id: i64) -> Vec<MessageItem> {
        let mut messages = Vec::new();
        let curr_user = self.inner.borrow().curr_user;
        let now = Local::now();

        if let Some(room) = self.inner.borrow().rooms.iter().find(|r| r.id == room_id) {
            let mut seen = AttrValue::default();

            for message in room.messages.iter() {
                let span = time_ago(&message.send_at, &now);
                if span != seen {
                    seen = span.clone();
                    messages.push(MessageItem::TimeDivider(span));
                }
                if message.sid == curr_user {
                    messages.push(MessageItem::Outgoing(MessageContent::from_attr(message)));
                } else {
                    messages.push(MessageItem::Incoming(MessageContent::from_attr(message)));
                }
            }
        }

        messages
    }

    pub fn get_relations(&self) -> HashMap<i64, Relation> {
        let mut map: HashMap<i64, Relation> = self
            .inner
            .borrow()
            .friends
            .iter()
            .map(|item| (item.id, item.relation.clone()))
            .collect();

        let curr_user = self.inner.borrow().curr_user;
        map.insert(curr_user, Relation::Yourself);
        map
    }

    pub fn get_members(&self, room_id: i64) -> Vec<MemberItem> {
        let mut members = Vec::new();
        let friends = self.get_relations();

        if let Some(room) = self.inner.borrow().rooms.iter().find(|r| r.id == room_id) {
            for member in room.members.iter() {
                let relation = friends
                    .get(&member.id)
                    .cloned()
                    .unwrap_or(Relation::Stranger);

                members.push(MemberItem::from_attr(member, relation))
            }
        }

        members
    }

    pub fn search_members(&self, room_id: i64, target: &String) -> Vec<MemberItem> {
        let mut members = Vec::new();
        let friends = self.get_relations();

        if let Some(room) = self.inner.borrow().rooms.iter().find(|r| r.id == room_id) {
            for member in room.members.iter() {
                if member.name.contains(target) {
                    let relation = friends
                        .get(&member.id)
                        .cloned()
                        .unwrap_or(Relation::Stranger);

                    members.push(MemberItem::from_attr(member, relation))
                }
            }
        }

        members
    }

    pub fn get_curr_friend(&self, friend_id: i64) -> Option<FriendAttr> {
        if let Some(friend) = self
            .inner
            .borrow()
            .friends
            .iter()
            .find(|x| x.id == friend_id)
        {
            Some(friend.clone())
        } else {
            None
        }
    }

    pub fn get_outgoing_friends(&self) -> Vec<FriendAttr> {
        let mut friends = Vec::new();

        for friend in self.inner.borrow().friends.iter() {
            if let Relation::OutgoingAdding = friend.relation {
                friends.push(friend.clone());
            }
        }

        friends
    }

    pub fn get_incoming_friends(&self) -> Vec<FriendAttr> {
        let mut friends = Vec::new();

        for friend in self.inner.borrow().friends.iter() {
            if let Relation::IncomingAdding = friend.relation {
                friends.push(friend.clone());
            }
        }

        friends
    }

    pub fn get_accepted_friends(&self) -> Vec<FriendItem> {
        let mut friends = Vec::new();

        for friend in self.inner.borrow().friends.iter() {
            if let Relation::Friend(_) = friend.relation {
                friends.push(FriendItem::from_attr(friend));
            }
        }

        friends
    }

    pub fn search_accepted_friends(&self, target: &String) -> Vec<FriendItem> {
        let mut friends = Vec::new();

        for friend in self.inner.borrow().friends.iter() {
            if let Relation::Friend(_) = friend.relation {
                if friend.username.contains(target) || friend.nickname.contains(target) {
                    friends.push(FriendItem::from_attr(friend));
                }
            }
        }

        friends
    }

    pub fn get_friend_from_user(&self, resp: &GetUserByNameResponse) -> Option<FriendAttr> {
        let friends = self.get_relations();

        if let Some(ref user) = resp.user {
            let relation = friends.get(&user.id).cloned().unwrap_or(Relation::Stranger);
            Some(FriendAttr::from_user(user, relation))
        } else {
            None
        }
    }

    // ------------------------ Tx ops ------------------------ //

    fn send(&self, msg: Message) {
        if let Some(ref tx) = self.inner.borrow_mut().tx {
            let mut tx = tx.clone();
            spawn_local(async move {
                tx.send(msg).await.unwrap_throw();
            })
        }
    }

    pub fn send_message(&self, req: NewMessageRequest) {
        let msg = ClientEvent::SendMessage(req).to_msg();
        self.send(msg);
    }

    pub fn get_user_rooms(&self) {
        let msg = ClientEvent::GetUserRooms.to_msg();
        self.send(msg);
    }

    pub fn create_room(&self, name: String, mut member_ids: Vec<i64>) {
        let curr_user = self.inner.borrow().curr_user;
        member_ids.insert(0, curr_user);
        let req = NewRoomRequest { name, member_ids };
        let msg = ClientEvent::CreateRoom(req).to_msg();
        self.send(msg);
    }

    pub fn delete_room(&self, room_id: i64) {
        let req = DeleteRoomRequest { room_id };
        let msg = ClientEvent::DeleteRoom(req).to_msg();
        self.send(msg);
    }

    pub fn update_room_name(&self, req: NewRoomNameResquest) {
        let msg = ClientEvent::UpdateRoomName(req).to_msg();
        self.send(msg);
    }

    pub fn leave_room(&self, room_id: i64) {
        let req = LeaveRoomRequest { room_id };
        let msg = ClientEvent::LeaveRoom(req).to_msg();
        self.send(msg);
    }

    pub fn add_members(&self, room_id: i64, member_ids: Vec<i64>) {
        let req = AddMembersRequest {
            room_id,
            member_ids,
        };
        let msg = ClientEvent::AddMembers(req).to_msg();
        self.send(msg);
    }

    pub fn delete_members(&self, req: DeleteMembersRequest) {
        let msg = ClientEvent::DeleteMembers(req).to_msg();
        self.send(msg);
    }

    pub fn add_friend(&self, friend_id: i64) {
        let req = AddFriendRequest { friend_id };
        let msg = ClientEvent::AddFriend(req).to_msg();
        self.send(msg);
    }

    pub fn accept_friend(&self, friend_id: i64) {
        let req = AcceptFriendRequest { friend_id };
        let msg = ClientEvent::AcceptFriend(req).to_msg();
        self.send(msg);
    }

    pub fn refuse_friend(&self, friend_id: i64) {
        let req = RefuseFriendRequest { friend_id };
        let msg = ClientEvent::RefuseFriend(req).to_msg();
        self.send(msg);
    }

    pub fn delete_friend(&self, friend_id: i64) {
        let req = DeleteFriendRequest { friend_id };
        let msg = ClientEvent::DeleteFriend(req).to_msg();
        self.send(msg);
    }
}
