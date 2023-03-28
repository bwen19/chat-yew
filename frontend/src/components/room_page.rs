use super::{
    common::Avatar, member_list::MemberList, message_list::MessageList, new_room::NewRoom,
    room_list::RoomList,
};
use crate::states::{
    chat::ChatState,
    room::{CurrRoomTrigger, RoomPageTrigger},
    types::CurrRoomItem,
};
use yew::{
    function_component, html, use_state_eq, Callback, Children, Html, Properties, UseStateHandle,
};

// ========================// RoomPage //======================== //

#[derive(PartialEq, Properties)]
pub struct RoomPageProps {
    pub chat: ChatState,
    pub trigger: RoomPageTrigger,
}

#[function_component]
pub fn RoomPage(props: &RoomPageProps) -> Html {
    let open_desc = use_state_eq(|| false);
    let nav_desc = {
        let open_desc = open_desc.clone();
        Callback::from(move |open: bool| open_desc.set(open))
    };

    let curr_room_id = use_state_eq(|| props.chat.curr_room());
    let nav_room = {
        let chat = props.chat.clone();
        let open_desc = open_desc.clone();
        let curr_room_id = curr_room_id.clone();

        Callback::from(move |room_id: i64| {
            chat.set_curr_room(room_id);
            open_desc.set(false);
            curr_room_id.set(room_id);
        })
    };

    html! {
        <>
        // middle room list
        <div class="shrink-0 h-full w-64 pt-3 border-r bg-slate-200 flex flex-col">
            <RoomList chat={props.chat.clone()} trigger={props.trigger.to_rooms()}
                curr_room_id={curr_room_id.clone()} nav_room={nav_room.clone()}/>
        </div>

        // right chat box
        {
            if *curr_room_id == -1 {
                html! {
                    <NewRoom chat={props.chat.clone()} trigger={props.trigger.to_new()} />
                }
            } else {
                html! {
                    <CurrRoom chat={props.chat.clone()} trigger={props.trigger.to_curr()} curr_room_id={curr_room_id.clone()}
                        open_desc={open_desc.clone()} nav_desc={nav_desc.clone()}>
                        <MessageList chat={props.chat.clone()} trigger={props.trigger.to_messages()} curr_room_id={curr_room_id.clone()} />
                    </CurrRoom>
                }
            }
        }

        // room members
        <RoomDesc chat={props.chat.clone()} trigger={props.trigger.to_curr()} curr_room_id={curr_room_id.clone()}
            open_desc={open_desc.clone()} nav_desc={nav_desc.clone()}>
            <MemberList chat={props.chat.clone()} trigger={props.trigger.to_members()}
                curr_room_id={curr_room_id.clone()}
                nav_room={nav_room.clone()} />
        </RoomDesc>

        </>
    }
}

// ========================// CurrRoom //======================== //

#[derive(PartialEq, Properties)]
struct CurrRoomProps {
    chat: ChatState,
    trigger: CurrRoomTrigger,
    curr_room_id: UseStateHandle<i64>,
    open_desc: UseStateHandle<bool>,
    nav_desc: Callback<bool>,
    children: Children,
}

#[function_component]
fn CurrRoom(props: &CurrRoomProps) -> Html {
    let room = props.chat.get_curr_room(*props.curr_room_id);

    let ontoggle = {
        let open_desc = props.open_desc.clone();
        let nav_desc = props.nav_desc.clone();
        move |_| nav_desc.emit(!*open_desc)
    };

    if let Some(room) = room {
        html! {
            <div class="grow h-full py-1 bg-slate-100 flex flex-col">
                // room header
                <div class="shrink-0 h-14 border-b px-4 flex items-center justify-between">
                    <div class="shrink-0 rounded-full">
                        <Avatar image={room.cover()} classes={"h-9 w-9"} />
                    </div>
                    <p class="text-slate-700 font-semibold">{room.name()}</p>
                    <div onclick={ontoggle}
                        class="w-9 h-9 rounded-full cursor-pointer text-slate-500 hover:text-sky-600 active:text-sky-500 hover:bg-slate-200 flex items-center justify-center">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                            stroke="currentColor" class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                        </svg>
                    </div>
                </div>

                {props.children.clone()}
            </div>
        }
    } else {
        html! {
            <div class="grow h-full py-1 bg-slate-100 flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.0"
                    stroke="currentColor" class="w-24 h-24 stroke-slate-300">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M8.625 12a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H8.25m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H12m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 01-2.555-.337A5.972 5.972 0 015.41 20.97a5.969 5.969 0 01-.474-.065 4.48 4.48 0 00.978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25z" />
                </svg>
            </div>
        }
    }
}

// ========================// RoomDesc //======================== //

#[derive(PartialEq, Properties)]
struct RoomDescProps {
    chat: ChatState,
    trigger: CurrRoomTrigger,
    curr_room_id: UseStateHandle<i64>,
    open_desc: UseStateHandle<bool>,
    nav_desc: Callback<bool>,
    children: Children,
}

#[function_component]
fn RoomDesc(props: &RoomDescProps) -> Html {
    let room_type = props.chat.get_curr_room(*props.curr_room_id);

    let onclose = {
        let nav_desc = props.nav_desc.clone();
        move |_| nav_desc.emit(false)
    };

    let show_description = |room_type: CurrRoomItem| match room_type {
        CurrRoomItem::Public(room) => html! {
            <>
            <div class="py-3 border-b flex flex-col items-center space-y-3">
                <div class="shrink-0 rounded-full">
                    <Avatar image={room.cover.clone()} classes={"h-20 w-20"} />
                </div>
                <p class="text-slate-700 font-semibold">{room.name.clone()}</p>
            </div>
            {props.children.clone()}
            </>
        },
        CurrRoomItem::Private(friend) => html! {
            <div class="p-3 flex flex-col items-center space-y-6">
                <div class="shrink-0 rounded-full">
                    <Avatar image={friend.avatar.clone()} classes={"h-20 w-20"} />
                </div>
                <div class="space-y-2">
                    <p class="text-center text-slate-700 text-lg font-semibold">
                        {friend.username}
                    </p>
                    <p class="text-center text-slate-600">
                        {friend.nickname.clone()}
                    </p>
                    <p class="text-center text-slate-500 text-sm">
                        {friend.bio.clone()}
                    </p>
                </div>
            </div>
        },
        CurrRoomItem::Personal(room) => html! {
            <div class="p-3 flex flex-col items-center space-y-6">
                <div class="shrink-0 rounded-full">
                    <Avatar image={room.cover.clone()} classes={"h-20 w-20"} />
                </div>
                <p class="text-center text-slate-700 text-lg font-semibold">
                    {room.name.clone()}
                </p>
                <p class="text-center text-slate-500 text-sm">
                    {room.desc.clone()}
                </p>
            </div>
        },
    };

    html! {
        <div hidden={!*props.open_desc} class="shrink-0 h-full w-56 border-l bg-slate-50">
            <div class="h-full flex flex-col">
                <div class="shrink-0 h-8 px-2 flex items-center justify-end">
                    <div onclick={onclose} class="cursor-pointer text-slate-500 hover:text-slate-800 active:text-slate-500">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                            stroke="currentColor" class="w-4 h-4">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </div>
                </div>

                {
                    if let Some(room_type) = room_type {
                        show_description(room_type)
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>

    }
}
