use super::{common::Avatar, search_box::SearchBox};
use crate::states::{chat::ChatState, room::RoomListTrigger, types::RoomItem};
use yew::{
    classes, function_component, html, use_memo, use_state, Callback, Html, Properties,
    UseStateHandle,
};

// ========================// RoomList //======================== //

#[derive(PartialEq, Properties)]
pub struct RoomListProps {
    pub chat: ChatState,
    pub trigger: RoomListTrigger,
    pub curr_room_id: UseStateHandle<i64>,
    pub nav_room: Callback<i64>,
}

#[function_component]
pub fn RoomList(props: &RoomListProps) -> Html {
    let rooms = use_memo(|_| props.chat.get_rooms(), props.trigger.clone());

    let onnewroom = {
        let nav_room = props.nav_room.clone();
        move |_| nav_room.emit(-1)
    };

    let searched = use_state(|| Vec::new());
    let searching = use_state(|| false);

    let onenter = {
        let chat = props.chat.clone();
        let searched = searched.clone();

        Callback::from(move |value: String| {
            if !(*value).is_empty() {
                let result = chat.search_rooms(&value);
                searched.set(result);
            }
        })
    };

    let show_room_list = if *searching {
        (*searched)
            .iter()
            .map(|item| {
                html! {
                    <RoomEntry key={item.id} room_item={item.clone()} nav_room={props.nav_room.clone()}
                        is_selected={*props.curr_room_id == item.id} />
                }
            })
            .collect::<Html>()
    } else {
        rooms
            .iter()
            .map(|item| {
                html! {
                    <RoomEntry key={item.id} room_item={item.clone()} nav_room={props.nav_room.clone()}
                        is_selected={*props.curr_room_id == item.id} />
                }
            })
            .collect::<Html>()
    };

    html! {
        <>
        <div class="px-3 my-2 w-full">
            <SearchBox searching={searching.clone()} onenter={onenter.clone()}
                placeholder={"Search rooms..."}/>
        </div>

        <div class="my-3 px-5 flex justify-between items-center">
            <p class="text-slate-500 font-semibold text-xs">
                {"CHAT ROOMS"}
                <span class="ml-2 text-sky-700 font-bold">
                    {if *searching {(*searched).len()} else {rooms.len()}}
                </span>
            </p>
            <div onclick={onnewroom}
                class="rounded-full p-2 cursor-pointer text-slate-500 hover:text-sky-600 hover:bg-slate-300 active:text-sky-500">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                    stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
            </div>
        </div>
        <div class="overflow-y-scroll hover:scrollbar no-scrollbar">
            <ul class="divide-y divide-slate-300">
                {show_room_list}
            </ul>
        </div>
        </>
    }
}

// ========================// RoomEntry //======================== //

#[derive(PartialEq, Properties)]
struct RoomEntryProps {
    room_item: RoomItem,
    is_selected: bool,
    nav_room: Callback<i64>,
}

#[function_component]
fn RoomEntry(props: &RoomEntryProps) -> Html {
    let onclick = {
        let room_id = props.room_item.id;
        let nav_room = props.nav_room.clone();
        move |_| nav_room.emit(room_id)
    };

    let show_unreads = move |unreads: i64| {
        let cls = "shrink-0 p-0.5 rounded-full px-1.5 bg-rose-500 text-xs text-slate-50".to_owned();

        html! {
            <p class={classes!(cls, (unreads - 1).is_negative().then(|| "invisible"))} >
                {unreads}
            </p>
        }
    };

    if props.is_selected {
        html! {
            <li class="group bg-sky-600 p-3 flex w-full items-start space-x-3 cursor-pointer">
                <div class="shrink-0 rounded-full group-hover:ring-2 group-hover:ring-sky-100">
                    <Avatar image={props.room_item.cover.clone()} classes="h-10 w-10" />
                </div>
                <div class="min-w-0 w-full">
                    <div class="flex items-center justify-between space-x-2">
                        <p class="truncate text-sm font-semibold text-white">
                            {props.room_item.name.clone()}
                        </p>
                        <p class="shrink-0 text-xs text-slate-50">
                            {props.room_item.latest_time.clone()}
                        </p>
                    </div>
                    <div class="mt-1 flex items-center justify-between space-x-2">
                        <p class="p-0.5 truncate text-xs text-slate-50 font-medium">
                            {props.room_item.latest_msg.clone()}
                        </p>

                        {show_unreads(0)}
                    </div>
                </div>
            </li>
        }
    } else {
        html! {
            <li {onclick} class="group hover:bg-sky-600 p-3 flex w-full items-start space-x-3 cursor-pointer">
                <div class="shrink-0 rounded-full">
                    <Avatar image={props.room_item.cover.clone()} classes="h-10 w-10" />
                </div>
                <div class="min-w-0 w-full">
                    <div class="flex items-center justify-between space-x-2">
                        <p class="truncate text-sm font-semibold text-slate-600 group-hover:text-white">
                            {props.room_item.name.clone()}
                        </p>
                        <p class="shrink-0 text-xs text-slate-400 group-hover:text-slate-50">
                            {props.room_item.latest_time.clone()}
                        </p>
                    </div>
                    <div class="mt-1 flex items-center justify-between space-x-2">
                        <p class="p-0.5 truncate text-xs text-slate-400 group-hover:text-slate-50 font-medium">
                            {props.room_item.latest_msg.clone()}
                        </p>

                        {show_unreads(props.room_item.unreads)}
                    </div>
                </div>
            </li>
        }
    }
}
