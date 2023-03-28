use super::{common::Avatar, search_box::SearchBox};
use crate::{
    api::config::ApiConfig,
    hooks::{use_request, UseRequestHandle},
    pages::ChatPage,
    states::{
        chat::ChatState,
        friend::NewFriendTrigger,
        types::{FriendAttr, Relation},
    },
};
use shared::user::{GetUserByNameRequest, GetUserByNameResponse};
use yew::{function_component, html, use_memo, use_state, Callback, Html, Properties};

// ========================// NewFriend //======================== //

#[derive(PartialEq, Properties)]
pub struct NewFriendProps {
    pub chat: ChatState,
    pub trigger: NewFriendTrigger,
    pub nav_page: Callback<ChatPage>,
}

#[function_component]
pub fn NewFriend(props: &NewFriendProps) -> Html {
    // show recieved friend invitation
    let incoming_friends = use_memo(|_| props.chat.get_incoming_friends(), props.trigger.clone());
    let accept_friend = {
        let chat = props.chat.clone();
        Callback::from(move |friend_id: i64| chat.accept_friend(friend_id))
    };
    let refuse_friend = {
        let chat = props.chat.clone();
        Callback::from(move |friend_id: i64| chat.refuse_friend(friend_id))
    };

    let show_incoming = if !incoming_friends.is_empty() {
        html! {
            <>
            <p class="mt-4 ml-2 text-sm text-slate-400 font-semibold">{"Incoming"}</p>
            <ul class="my-2 mx-2 space-y-1">
                {
                    incoming_friends.iter().map(|item| {
                        html! {
                            <IncomingFriendEntry key={item.id} friend={item.clone()}
                                accept_friend={accept_friend.clone()} refuse_friend={refuse_friend.clone()} />
                        }
                    }).collect::<Html>()
                }
            </ul>
            </>
        }
    } else {
        html! {}
    };

    // show sent friend invitation
    let outgoing_friends = use_memo(|_| props.chat.get_outgoing_friends(), props.trigger.clone());
    let show_outgoing = if !outgoing_friends.is_empty() {
        html! {
            <>
            <p class="mt-4 ml-2 text-sm text-slate-400 font-semibold">{"Outgoing"}</p>
            <ul class="my-2 mx-2 space-y-1">
                {
                    outgoing_friends.iter().map(|item| {
                        html! {
                            <OutgoingFriendEntry key={item.id} friend={item.clone()} />
                        }
                    }).collect::<Html>()
                }
            </ul>
            </>
        }
    } else {
        html! {}
    };

    html! {
        <div class="grow h-full bg-slate-100">
            <div class="shrink-0 h-14 border-b px-4 flex items-center justify-center">
                <p class="text-slate-600 font-semibold">{"New friend"}</p>
            </div>
            <div class="max-w-md mx-auto p-6">
                <FindNewFriend chat={props.chat.clone()} trigger={props.trigger.clone()} nav_page={props.nav_page.clone()} />

                <div class="overflow-y-scroll hover:scrollbar no-scrollbar">
                    {show_incoming}
                    {show_outgoing}
                </div>
            </div>
        </div>
    }
}

// ========================// FindNewFriend //======================== //

#[derive(PartialEq, Properties)]
struct FindNewFriendProps {
    chat: ChatState,
    trigger: NewFriendTrigger,
    nav_page: Callback<ChatPage>,
}

#[function_component]
fn FindNewFriend(props: &FindNewFriendProps) -> Html {
    let find_user: UseRequestHandle<GetUserByNameRequest, GetUserByNameResponse> =
        use_request(ApiConfig::GetUserByName);

    let searching = use_state(|| false);
    let finding = (*find_user)
        .as_ref()
        .map(|u| props.chat.get_friend_from_user(&u))
        .flatten();

    let onenter = {
        let find_user = find_user.clone();

        Callback::from(move |value: String| {
            if !(*value).is_empty() {
                let req = GetUserByNameRequest { username: value };
                find_user.send(req);
            }
        })
    };

    let onadding = {
        let chat = props.chat.clone();
        let friend_id = finding.clone().map(|x| x.id).unwrap_or(0);
        move |_| chat.add_friend(friend_id)
    };

    let show_operation = move |relation: Relation| match relation {
        Relation::Stranger => {
            html! {
                <div onclick={onadding} class="cursor-pointer text-slate-500 hover:text-sky-600 active:text-sky-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                        stroke-width="2" stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z" />
                    </svg>
                </div>
            }
        }
        Relation::Friend(room_id) => {
            let onchat = {
                let nav_page = props.nav_page.clone();
                let chat = props.chat.clone();

                move |_| {
                    chat.set_curr_room(room_id);
                    nav_page.emit(ChatPage::Room);
                }
            };

            html! {
                <div onclick={onchat} class="cursor-pointer text-slate-500 hover:text-sky-600 active:text-sky-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                        stroke-width="2" stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M8.625 12a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H8.25m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H12m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 01-2.555-.337A5.972 5.972 0 015.41 20.97a5.969 5.969 0 01-.474-.065 4.48 4.48 0 00.978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25z" />
                    </svg>
                </div>
            }
        }
        _ => {
            html! {
                <div class="text-slate-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                        stroke-width="2" stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z" />
                    </svg>
                </div>
            }
        }
    };

    let show_finding = if *searching {
        if let Some(ref finding) = finding {
            html! {
                <div class="group p-2 m-2 rounded-md cursor-default hover:shadow-sm hover:shadow-slate-400 flex items-center space-x-3">
                    <div class="shrink-0 rounded-full">
                        <Avatar image={finding.avatar.clone()} classes="h-8 w-8" />
                    </div>
                    <span class="truncate w-28 text-slate-600 text-sm font-semibold">
                        {finding.username.clone()}
                    </span>
                    <span class="truncate grow text-slate-500 text-sm font-semibold">
                        {finding.nickname.clone()}
                    </span>
                    {show_operation(finding.relation.clone())}
                </div>
            }
        } else {
            html! {
                <div class="p-2 m-2 rounded-md cursor-default text-center font-semibold text-slate-400">
                    {"Nothing to find"}
                </div>
            }
        }
    } else {
        html! {}
    };

    html! {
        <>
        <div class="mx-2">
            <SearchBox searching={searching.clone()} onenter={onenter.clone()}
                placeholder={"Find new friend..."}/>
        </div>
        {show_finding}
        </>
    }
}

// ========================// IncomingFriendEntry //======================== //

#[derive(PartialEq, Properties)]
struct IncomingFriendEntryProps {
    friend: FriendAttr,
    accept_friend: Callback<i64>,
    refuse_friend: Callback<i64>,
}

#[function_component]
fn IncomingFriendEntry(props: &IncomingFriendEntryProps) -> Html {
    let onaccept = {
        let accept_friend = props.accept_friend.clone();
        let friend_id = props.friend.id;
        move |_| accept_friend.emit(friend_id)
    };
    let onrefuse = {
        let refuse_friend = props.refuse_friend.clone();
        let friend_id = props.friend.id;
        move |_| refuse_friend.emit(friend_id)
    };

    html! {
        <li class="group p-2 space-x-3 rounded-md cursor-default hover:shadow-sm hover:shadow-slate-400 flex items-center">
            <div class="shrink-0 rounded-full">
                <Avatar image={props.friend.avatar.clone()} classes="h-8 w-8" />
            </div>
            <span class="truncate w-28 text-slate-600 text-sm font-semibold">
                {props.friend.username.clone()}
            </span>
            <span class="truncate grow text-slate-500 text-sm font-semibold">
                {props.friend.nickname.clone()}
            </span>
            <div onclick={onrefuse} class="cursor-pointer p-1 text-slate-500 hover:text-rose-500 active:text-rose-400">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                    stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
            </div>
            <div onclick={onaccept} class="cursor-pointer p-1 text-slate-500 hover:text-sky-600 active:text-sky-500">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                    stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
            </div>
        </li>
    }
}

// ========================// OutgoingFriendEntry //======================== //

#[derive(PartialEq, Properties)]
struct OutgoingFriendEntryProps {
    friend: FriendAttr,
}

#[function_component]
fn OutgoingFriendEntry(props: &OutgoingFriendEntryProps) -> Html {
    // TODO: add cancel method here
    let oncancel = { move |_| {} };

    html! {
        <li class="group p-2 space-x-3 rounded-md cursor-default hover:shadow-sm hover:shadow-slate-400 flex items-center">
            <div class="shrink-0 rounded-full">
                <Avatar image={props.friend.avatar.clone()} classes="h-8 w-8" />
            </div>
            <span class="truncate w-28 text-slate-600 text-sm font-semibold">
                {props.friend.username.clone()}
            </span>
            <span class="truncate grow text-slate-500 text-sm font-semibold">
                {props.friend.nickname.clone()}
            </span>
            <div onclick={oncancel} class="cursor-pointer p-1 text-slate-500 hover:text-rose-500 active:text-rose-400">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                    stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3" />
                </svg>
            </div>
        </li>
    }
}
