use super::{
    trigger::{AffectedFriend, AffectedRoom, TriggerAction, Trigger},
    types::{FriendAttr, MemberAttr, RoomAttr, cmp_room},
};

use futures::channel::mpsc;
use gloo_net::websocket::Message;
use shared::event::ServerEvent;

// ========================// WebSocketState //======================== //

pub struct WebSocketState {
    pub rooms: Vec<RoomAttr>,
    pub friends: Vec<FriendAttr>,
    pub curr_room: i64,
    pub curr_friend: i64,
    pub curr_user: i64,
    pub tx: Option<mpsc::Sender<Message>>,
}

impl WebSocketState {
    pub fn new(curr_user: i64) -> Self {
        Self {
            rooms: Vec::new(),
            friends: Vec::new(),
            curr_room: 0,
            curr_friend: 0,
            curr_user,
            tx: None,
        }
    }

    pub fn handle_event(&mut self, event: ServerEvent, trigger: Trigger) -> Result<(), String> {
        match event {
            ServerEvent::Close(msg) => {
                return Err(msg);
            }
            ServerEvent::Initialized(mut resp) => {
                resp.rooms.sort_by(cmp_room);
                let mut rooms: Vec<RoomAttr> = resp.rooms.into_iter().map(RoomAttr::from).collect();
                self.rooms.clear();
                self.rooms.append(&mut rooms);

                let mut friends: Vec<FriendAttr> =
                    resp.friends.into_iter().map(FriendAttr::from).collect();
                self.friends.clear();
                self.friends.append(&mut friends);

                trigger.dispatch(TriggerAction::Init);
            }
            ServerEvent::ReceiveMessage(resp) => {
                if let Some(idx) = self.rooms.iter().position(|room| room.id == resp.room_id) {
                    let mut room = self.rooms.remove(idx);
                    room.messages.push(resp.message.into());
                    room.unreads += 1;

                    let aroom = AffectedRoom {
                        room_id: room.id,
                        curr_room: self.curr_room,
                    };

                    self.rooms.push(room);
                    trigger.dispatch(TriggerAction::Message(aroom));
                }
            }
            ServerEvent::UserRooms(mut resp) => {
                resp.rooms.sort_by(cmp_room);
                let mut rooms: Vec<RoomAttr> = resp.rooms.into_iter().map(RoomAttr::from).collect();

                self.rooms.clear();
                self.rooms.append(&mut rooms);

                trigger.dispatch(TriggerAction::Init);
            }
            ServerEvent::JoinedRoom(resp) => {
                let aroom = AffectedRoom {
                    room_id: resp.room.id,
                    curr_room: self.curr_room,
                };

                self.rooms.push(resp.room.into());
                trigger.dispatch(TriggerAction::Room(aroom));
            }
            ServerEvent::DeletedRoom(resp) => {
                if let Some(idx) = self.rooms.iter().position(|room| room.id == resp.room_id) {
                    self.rooms.remove(idx);

                    let aroom = AffectedRoom {
                        room_id: resp.room_id,
                        curr_room: self.curr_room,
                    };
                    trigger.dispatch(TriggerAction::Room(aroom));
                }
            }
            ServerEvent::UpdatedRoomName(resp) => {
                if let Some(room) = self.rooms.iter_mut().find(|room| room.id == resp.room_id) {
                    room.name = resp.name.into();

                    let aroom = AffectedRoom {
                        room_id: resp.room_id,
                        curr_room: self.curr_room,
                    };
                    trigger.dispatch(TriggerAction::Room(aroom));
                }
            }
            ServerEvent::LeavedRoom(resp) => {
                self.rooms.retain(|x| x.id != resp.room_id);

                let aroom = AffectedRoom {
                    room_id: resp.room_id,
                    curr_room: self.curr_room,
                };
                trigger.dispatch(TriggerAction::Room(aroom));
            }
            ServerEvent::AddedRoomMembers(resp) => {
                if let Some(room) = self.rooms.iter_mut().find(|room| room.id == resp.room_id) {
                    let mut members: Vec<MemberAttr> =
                        resp.members.into_iter().map(MemberAttr::from).collect();
                    room.members.append(&mut members);

                    let aroom = AffectedRoom {
                        room_id: resp.room_id,
                        curr_room: self.curr_room,
                    };
                    trigger.dispatch(TriggerAction::Member(aroom));
                }
            }
            ServerEvent::DeletedRoomMembers(resp) => {
                if let Some(room) = self.rooms.iter_mut().find(|room| room.id == resp.room_id) {
                    room.members.retain(|x| !resp.member_ids.contains(&x.id));

                    let aroom = AffectedRoom {
                        room_id: resp.room_id,
                        curr_room: self.curr_room,
                    };
                    trigger.dispatch(TriggerAction::Member(aroom));
                }
            }
            ServerEvent::UserFriends(resp) => {
                let mut friends: Vec<FriendAttr> =
                    resp.friends.into_iter().map(FriendAttr::from).collect();
                self.friends.clear();
                self.friends.append(&mut friends);

                trigger.dispatch(TriggerAction::Init);
            }
            ServerEvent::AddFriend(resp) => {
                self.friends.push(resp.friend.into());

                trigger.dispatch(TriggerAction::NewFriend);
            }
            ServerEvent::AcceptedFriend(resp) => {
                if let Some(idx) = self.friends.iter().position(|x| x.id == resp.friend.id) {
                    self.friends.swap_remove(idx);
                }

                let afriend = AffectedFriend {
                    friend_id: resp.friend.id,
                    curr_friend: self.curr_friend,
                };

                self.friends.push(resp.friend.into());
                trigger.dispatch(TriggerAction::Friend(afriend));
            }
            ServerEvent::RefusedFriend(resp) => {
                if let Some(idx) = self.friends.iter().position(|x| x.id == resp.friend_id) {
                    self.friends.swap_remove(idx);

                    trigger.dispatch(TriggerAction::NewFriend);
                }
            }
            ServerEvent::DeletedFriend(resp) => {
                if let Some(idx) = self.friends.iter().position(|x| x.id == resp.friend_id) {
                    self.friends.swap_remove(idx);

                    let afriend = AffectedFriend {
                        friend_id: resp.friend_id,
                        curr_friend: self.curr_friend,
                    };
                    trigger.dispatch(TriggerAction::Friend(afriend));
                }
            }
        }
        Ok(())
    }
}
