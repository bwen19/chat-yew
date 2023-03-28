use super::{common::Avatar, search_box::SearchBox};
use crate::states::{chat::ChatState, friend::FriendListTrigger, types::FriendItem};
use yew::{
    function_component, html, use_memo, use_state, Callback, Html, Properties, UseStateHandle,
};

// ========================// FriendList //======================== //

#[derive(PartialEq, Properties)]
pub struct FriendListProps {
    pub chat: ChatState,
    pub trigger: FriendListTrigger,
    pub curr_friend_id: UseStateHandle<i64>,
    pub nav_friend: Callback<i64>,
}

#[function_component]
pub fn FriendList(props: &FriendListProps) -> Html {
    let friends = use_memo(|_| props.chat.get_accepted_friends(), props.trigger.clone());

    let onnewfriend = {
        let nav_friend = props.nav_friend.clone();
        move |_| nav_friend.emit(-1)
    };

    let searching = use_state(|| false);
    let searched = use_state(|| Vec::new());

    let onenter = {
        let chat = props.chat.clone();
        let searched = searched.clone();

        Callback::from(move |value: String| {
            if !(*value).is_empty() {
                let result = chat.search_accepted_friends(&value);
                searched.set(result);
            }
        })
    };

    let show_friend_list = if *searching {
        (*searched)
            .iter()
            .map(|item| {
                html! {
                    <FriendEntry key={item.id} friend={item.clone()} nav_friend={props.nav_friend.clone()}
                        is_selected={*props.curr_friend_id == item.id} />
                }
            })
            .collect::<Html>()
    } else {
        friends
            .iter()
            .map(|item| {
                html! {
                    <FriendEntry key={item.id} friend={item.clone()} nav_friend={props.nav_friend.clone()}
                        is_selected={*props.curr_friend_id == item.id} />
                }
            })
            .collect::<Html>()
    };

    html! {
        <>
        <div class="px-3 my-2 w-full">
            <SearchBox searching={searching.clone()} onenter={onenter.clone()}
                placeholder={"Search friends..."}/>
        </div>

        <div class="my-3 px-5 flex justify-between items-center">
            <p class="text-slate-500 font-semibold text-xs">
                {"MY FRIENDS"}
                <span class="ml-2 text-sky-600 font-bold">
                    {if *searching {(*searched).len()} else {friends.len()}}
                </span>
            </p>
            <div onclick={onnewfriend} class="cursor-pointer rounded-full p-2 text-slate-500 hover:text-sky-600 hover:bg-slate-300 active:text-sky-500">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                    stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z" />
                </svg>
            </div>
        </div>
        <div class="overflow-y-scroll hover:scrollbar no-scrollbar">
            <ul class="divide-y divide-slate-300">
                {show_friend_list}
            </ul>
        </div>
        </>
    }
}

// ========================// FriendEntry //======================== //

#[derive(PartialEq, Properties)]
struct FriendEntryProps {
    friend: FriendItem,
    is_selected: bool,
    nav_friend: Callback<i64>,
}

#[function_component]
fn FriendEntry(props: &FriendEntryProps) -> Html {
    let onclick = {
        let friend_id = props.friend.id;
        let nav_friend = props.nav_friend.clone();
        Callback::from(move |_| {
            nav_friend.emit(friend_id);
        })
    };

    if props.is_selected {
        html! {
            <li class="group bg-sky-600 cursor-pointer p-3 flex w-full items-center space-x-3">
                <div class="shrink-0 rounded-full group-hover:ring-2 group-hover:ring-sky-100">
                    <Avatar image={props.friend.avatar.clone()} classes="h-10 w-10" />
                </div>
                <div class="min-w-0 truncate text-slate-50 font-semibold ">
                    {props.friend.nickname.clone()}
                </div>
            </li>
        }
    } else {
        html! {
            <li {onclick} class="group hover:bg-sky-600 cursor-pointer p-3 flex w-full items-center space-x-3">
                <div class="shrink-0 rounded-full">
                    <Avatar image={props.friend.avatar.clone()} classes="h-10 w-10" />
                </div>
                <div class="min-w-0 truncate text-slate-600 group-hover:text-slate-50 font-semibold ">
                    {props.friend.nickname.clone()}
                </div>
            </li>
        }
    }
}
