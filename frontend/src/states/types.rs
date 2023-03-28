use chrono::{DateTime, Datelike, Local, Utc};
use gloo_net::websocket::Message;
use shared::{
    event::ClientEvent, friend::FriendInfo, member::MemberInfo, message::MessageInfo,
    room::RoomInfo, user::UserInfo,
};
use std::cmp::Ordering;
use wasm_bindgen::UnwrapThrowExt;
use yew::AttrValue;

// ========================// Types //======================== //

// ---------------- ConvertToMessage ---------------- //
pub trait ConvertToMessage {
    fn to_msg(&self) -> Message;
}

impl ConvertToMessage for ClientEvent {
    fn to_msg(&self) -> Message {
        let bs = serde_json::to_vec(self).unwrap_throw();
        Message::Bytes(bs)
    }
}

// ---------------- Relation ---------------- //
#[derive(Clone, PartialEq)]
pub enum Relation {
    Yourself,
    Friend(i64),
    OutgoingAdding,
    IncomingAdding,
    Stranger,
}

impl Relation {
    pub fn from_info(friend: &FriendInfo) -> Self {
        match friend.status.as_str() {
            "accepted" => Self::Friend(friend.room_id),
            "adding" => {
                if friend.first {
                    Self::OutgoingAdding
                } else {
                    Self::IncomingAdding
                }
            }
            _ => Self::Stranger,
        }
    }
}

// ========================// WebSocket Types //======================== //

// ---------------- FriendAttr ---------------- //
#[derive(Clone, PartialEq)]
pub struct FriendAttr {
    pub id: i64,
    pub username: AttrValue,
    pub nickname: AttrValue,
    pub avatar: AttrValue,
    pub bio: AttrValue,
    pub relation: Relation,
}

impl From<FriendInfo> for FriendAttr {
    fn from(friend: FriendInfo) -> Self {
        Self {
            id: friend.id,
            relation: Relation::from_info(&friend),
            username: friend.username.into(),
            nickname: friend.nickname.into(),
            avatar: friend.avatar.into(),
            bio: friend.bio.into(),
        }
    }
}

impl FriendAttr {
    pub fn from_user(user: &UserInfo, relation: Relation) -> Self {
        Self {
            id: user.id,
            username: user.username.clone().into(),
            nickname: user.nickname.clone().into(),
            avatar: user.avatar.clone().into(),
            bio: user.avatar.clone().into(),
            relation,
        }
    }
}

// ---------------- MessageAttr ---------------- //

pub struct MessageAttr {
    pub id: i64,
    pub sid: i64,
    pub name: AttrValue,
    pub avatar: AttrValue,
    pub content: AttrValue,
    pub kind: AttrValue,
    pub send_at: DateTime<Utc>,
}

impl From<MessageInfo> for MessageAttr {
    fn from(message: MessageInfo) -> Self {
        Self {
            id: message.id,
            sid: message.sid,
            name: message.name.into(),
            avatar: message.avatar.into(),
            content: message.content.into(),
            kind: message.kind.into(),
            send_at: message.send_at,
        }
    }
}

// ---------------- MemberAttr ---------------- //
pub struct MemberAttr {
    pub id: i64,
    pub name: AttrValue,
    pub avatar: AttrValue,
    pub rank: AttrValue,
}

impl From<MemberInfo> for MemberAttr {
    fn from(member: MemberInfo) -> Self {
        Self {
            id: member.id,
            name: member.name.into(),
            avatar: member.avatar.into(),
            rank: member.rank.into(),
        }
    }
}

// ---------------- RoomAttr ---------------- //
pub struct RoomAttr {
    pub id: i64,
    pub name: AttrValue,
    pub cover: AttrValue,
    pub category: AttrValue,
    pub unreads: i64,
    pub members: Vec<MemberAttr>,
    pub messages: Vec<MessageAttr>,
}

impl From<RoomInfo> for RoomAttr {
    fn from(room: RoomInfo) -> Self {
        Self {
            id: room.id,
            name: room.name.into(),
            cover: room.cover.into(),
            category: room.category.into(),
            unreads: 0,
            members: room.members.into_iter().map(MemberAttr::from).collect(),
            messages: room.messages.into_iter().map(MessageAttr::from).collect(),
        }
    }
}

// ========================// Display Types //======================== //

// ---------------- RoomItem ---------------- //
#[derive(PartialEq, Clone)]
pub struct RoomItem {
    pub id: i64,
    pub name: AttrValue,
    pub cover: AttrValue,
    pub unreads: i64,
    pub latest_msg: AttrValue,
    pub latest_time: AttrValue,
}

impl RoomItem {
    pub fn from_attr(room: &RoomAttr, curr_user: i64) -> Self {
        let (name, cover) = parse_room_meta(&room, curr_user);
        let (latest_msg, latest_time) = latest_message(&room);
        Self {
            id: room.id,
            name,
            cover,
            unreads: room.unreads,
            latest_msg,
            latest_time,
        }
    }
}

// ---------------- CurrRoomItem ---------------- //
#[derive(Clone, PartialEq)]
pub struct PublicRoom {
    pub id: i64,
    pub name: AttrValue,
    pub cover: AttrValue,
    pub rank: AttrValue,
}

impl PublicRoom {
    pub fn from_attr(room: &RoomAttr, curr_user: i64) -> Self {
        let rank = room
            .members
            .iter()
            .find(|x| x.id == curr_user)
            .map(|x| x.rank.clone().into())
            .unwrap_or(AttrValue::from("member"));

        Self {
            id: room.id,
            name: room.name.clone(),
            cover: room.cover.clone(),
            rank,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct PersonalRoom {
    pub id: i64,
    pub name: AttrValue,
    pub cover: AttrValue,
    pub desc: AttrValue,
}

impl PersonalRoom {
    pub fn from_attr(room: &RoomAttr) -> Self {
        Self {
            id: room.id,
            name: room.name.clone(),
            cover: room.cover.clone(),
            desc: AttrValue::from("Blank"),
        }
    }
}

#[derive(Clone)]
pub enum CurrRoomItem {
    Public(PublicRoom),
    Private(FriendAttr),
    Personal(PersonalRoom),
}

impl CurrRoomItem {
    pub fn name(&self) -> AttrValue {
        match self {
            CurrRoomItem::Public(room) => room.name.clone(),
            CurrRoomItem::Private(friend) => friend.nickname.clone(),
            CurrRoomItem::Personal(room) => room.name.clone(),
        }
    }

    pub fn cover(&self) -> AttrValue {
        match self {
            CurrRoomItem::Public(room) => room.cover.clone(),
            CurrRoomItem::Private(friend) => friend.avatar.clone(),
            CurrRoomItem::Personal(room) => room.cover.clone(),
        }
    }
}

// ---------------- MessageItem ---------------- //
#[derive(Clone)]
pub struct MessageContent {
    pub id: i64,
    pub sid: i64,
    pub name: AttrValue,
    pub avatar: AttrValue,
    pub content: AttrValue,
    pub kind: AttrValue,
    pub send_at: AttrValue,
}

impl MessageContent {
    pub fn from_attr(message: &MessageAttr) -> Self {
        Self {
            id: message.id,
            sid: message.sid,
            name: message.name.clone(),
            avatar: message.avatar.clone(),
            content: message.content.clone(),
            kind: message.kind.clone(),
            send_at: time_local(&message.send_at),
        }
    }
}

#[derive(Clone)]
pub enum MessageItem {
    TimeDivider(AttrValue),
    Incoming(MessageContent),
    Outgoing(MessageContent),
}

impl PartialEq for MessageItem {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl MessageItem {
    pub fn key(&self) -> AttrValue {
        match self {
            MessageItem::TimeDivider(s) => s.clone(),
            MessageItem::Incoming(content) => format!("{}", content.id).into(),
            MessageItem::Outgoing(content) => format!("{}", content.id).into(),
        }
    }
}

// ---------------- MemberItem ---------------- //

#[derive(Clone, PartialEq)]
pub struct MemberItem {
    pub id: i64,
    pub name: AttrValue,
    pub avatar: AttrValue,
    pub rank: AttrValue,
    pub relation: Relation,
}

impl MemberItem {
    pub fn from_attr(member: &MemberAttr, relation: Relation) -> Self {
        Self {
            id: member.id,
            name: member.name.clone(),
            avatar: member.avatar.clone(),
            rank: member.rank.clone(),
            relation,
        }
    }
}

// ---------------- FriendItem ---------------- //
#[derive(Clone, PartialEq)]
pub struct FriendItem {
    pub id: i64,
    pub nickname: AttrValue,
    pub avatar: AttrValue,
}

impl FriendItem {
    pub fn from_attr(friend: &FriendAttr) -> Self {
        Self {
            id: friend.id,
            nickname: friend.nickname.clone(),
            avatar: friend.avatar.clone(),
        }
    }
}

// ========================// UTILS //======================== //

/// Display short local time
pub fn time_local(time: &DateTime<Utc>) -> AttrValue {
    let time = time.with_timezone(&Local);
    time.format("%H:%M").to_string().into()
}

/// Display elapsed time
pub fn time_ago(time: &DateTime<Utc>, now: &DateTime<Local>) -> AttrValue {
    let time = time.with_timezone(&Local);

    if now.year() - time.year() > 0 {
        time.format("%-d %B, %Y").to_string().into()
    } else if now.month() - time.month() > 0 {
        time.format("%A, %-d %B").to_string().into()
    } else {
        let days = now.day() - time.day();
        match days {
            0 => "Today".into(),
            1 => "Yesterday".into(),
            _ => time.format("%A, %-d %B").to_string().into(),
        }
    }
}

/// Display elapsed time shortly
pub fn time_ago_short(time: &DateTime<Utc>, now: &DateTime<Local>) -> AttrValue {
    let time = time.with_timezone(&Local);

    if now.year() - time.year() > 0 {
        time.format("%Y-%m-%d").to_string().into()
    } else if now.month() - time.month() > 0 {
        time.format("%m-%d").to_string().into()
    } else {
        let days = now.day() - time.day();
        match days {
            0 => time.format("%H:%M").to_string().into(),
            1 => "Yesterday".into(),
            _ => time.format("%m-%d").to_string().into(),
        }
    }
}

/// compare two rooms by the latest message
pub fn cmp_room(a: &RoomInfo, b: &RoomInfo) -> Ordering {
    let a = a.messages.last();
    let b = b.messages.last();

    if let Some(a) = a {
        if let Some(b) = b {
            a.send_at.cmp(&b.send_at)
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Less
    }
}

/// Parse the name and cover of room
pub fn parse_room_meta(room: &RoomAttr, curr_user: i64) -> (AttrValue, AttrValue) {
    match room.category.as_str() {
        "private" => {
            if let Some(member) = room.members.iter().find(|x| x.id != curr_user) {
                (member.name.clone(), member.avatar.clone())
            } else {
                (room.name.clone(), room.cover.clone())
            }
        }
        _ => (room.name.clone(), room.cover.clone()),
    }
}

/// Return the latest message (content, time) of the room
pub fn latest_message(room: &RoomAttr) -> (AttrValue, AttrValue) {
    if let Some(msg) = room.messages.last() {
        let content = match room.category.as_str() {
            "public" => format!("{}: {}", msg.name, msg.content).into(),
            _ => msg.content.clone(),
        };

        let now = Local::now();
        let time = time_ago_short(&msg.send_at, &now);

        (content, time)
    } else {
        ("".into(), "".into())
    }
}
