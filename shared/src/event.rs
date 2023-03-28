use crate::{
    friend::{
        AcceptFriendRequest, AcceptFriendResponse, AddFriendRequest, AddFriendResponse,
        DeleteFriendRequest, DeleteFriendResponse, RefuseFriendRequest, RefuseFriendResponse,
        UserFriendsResponse,
    },
    member::{AddMembersRequest, AddMembersResponse, DeleteMembersRequest, DeleteMembersResponse},
    message::{InitialResponse, NewMessageRequest, NewMessageResponse},
    room::{
        DeleteRoomRequest, DeleteRoomResponse, LeaveRoomRequest, LeaveRoomResponse,
        NewRoomNameResponse, NewRoomNameResquest, NewRoomRequest, NewRoomResponse,
        UserRoomsResponse,
    },
};
use serde::{Deserialize, Serialize};

// ========================// ClientEvent //======================== //

/// Events from client to server
#[derive(Deserialize, Serialize)]
pub enum ClientEvent {
    Close,
    Initialization,
    SendMessage(NewMessageRequest),
    // Room
    GetUserRooms,
    CreateRoom(NewRoomRequest),
    DeleteRoom(DeleteRoomRequest),
    UpdateRoomName(NewRoomNameResquest),
    // member
    LeaveRoom(LeaveRoomRequest),
    AddMembers(AddMembersRequest),
    DeleteMembers(DeleteMembersRequest),
    // Friend
    GetUserFriends,
    AddFriend(AddFriendRequest),
    AcceptFriend(AcceptFriendRequest),
    RefuseFriend(RefuseFriendRequest),
    DeleteFriend(DeleteFriendRequest),
}

// ========================// ServerEvent //======================== //

/// Events from server to client
#[derive(Deserialize, Serialize)]
pub enum ServerEvent {
    Close(String),
    Initialized(InitialResponse),
    ReceiveMessage(NewMessageResponse),
    // Room
    UserRooms(UserRoomsResponse),
    JoinedRoom(NewRoomResponse),
    DeletedRoom(DeleteRoomResponse),
    UpdatedRoomName(NewRoomNameResponse),
    // Member
    LeavedRoom(LeaveRoomResponse),
    AddedRoomMembers(AddMembersResponse),
    DeletedRoomMembers(DeleteMembersResponse),
    // Friend
    UserFriends(UserFriendsResponse),
    AddFriend(AddFriendResponse),
    AcceptedFriend(AcceptFriendResponse),
    RefusedFriend(RefuseFriendResponse),
    DeletedFriend(DeleteFriendResponse),
}
