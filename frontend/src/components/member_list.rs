use super::{common::Avatar, search_box::SearchBox};
use crate::states::{
    chat::ChatState,
    room::MemberListTrigger,
    types::{FriendItem, MemberItem, Relation},
};
use yew::{
    classes, function_component, html, use_memo, use_state, AttrValue, Callback, Html, Properties,
    UseStateHandle,
};

// ========================// MemberList //======================== //

#[derive(PartialEq, Properties)]
pub struct MemberListProps {
    pub chat: ChatState,
    pub trigger: MemberListTrigger,
    pub curr_room_id: UseStateHandle<i64>,
    pub nav_room: Callback<i64>,
}

#[function_component]
pub fn MemberList(props: &MemberListProps) -> Html {
    let rank = use_memo(
        |_| props.chat.get_rank(*props.curr_room_id),
        props.trigger.clone(),
    );

    let open_adding = use_state(|| false);
    let onopen = {
        let open_adding = open_adding.clone();
        move |_| open_adding.set(true)
    };
    let onclose = {
        let open_adding = open_adding.clone();
        move |_| open_adding.set(false)
    };

    let show_switch = if *rank == "owner" || *rank == "manager" {
        if *open_adding {
            html! {
                <div onclick={onclose}
                    class="rounded-full p-2 cursor-pointer text-slate-500 hover:text-sky-600 hover:bg-slate-200 active:text-sky-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3" />
                    </svg>
                </div>
            }
        } else {
            html! {
                <div onclick={onopen}
                    class="rounded-full p-2 cursor-pointer text-slate-500 hover:text-sky-600 hover:bg-slate-200 active:text-sky-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z" />
                    </svg>
                </div>
            }
        }
    } else {
        html! {}
    };

    // show members or searched list
    let members = use_memo(
        |_| props.chat.get_members(*props.curr_room_id),
        props.trigger.clone(),
    );

    let searching = use_state(|| false);
    let searched = use_state(Vec::new);

    let onenter = {
        let chat = props.chat.clone();
        let searched = searched.clone();
        let room_id = *props.curr_room_id;

        Callback::from(move |value: String| {
            if !(*value).is_empty() {
                let result = chat.search_members(room_id, &value);
                searched.set(result);
            }
        })
    };

    let add_friend = {
        let chat = props.chat.clone();
        Callback::from(move |friend_id: i64| chat.add_friend(friend_id))
    };

    let show_member_list = if *searching {
        (*searched)
            .iter()
            .map(|item| {
                html! {
                    <MemberEntry key={item.id} member={item.clone()}
                        nav_room={props.nav_room.clone()} add_friend={add_friend.clone()} />
                }
            })
            .collect::<Html>()
    } else {
        members
            .iter()
            .map(|item| {
                html! {
                    <MemberEntry key={item.id} member={item.clone()}
                        nav_room={props.nav_room.clone()} add_friend={add_friend.clone()} />
                }
            })
            .collect::<Html>()
    };

    // the operation button at the bottom
    let ondelete = {
        let chat = props.chat.clone();
        let room_id = *props.curr_room_id;
        move |_| chat.delete_room(room_id)
    };
    let onleave = {
        let chat = props.chat.clone();
        let room_id = *props.curr_room_id;
        move |_| chat.leave_room(room_id)
    };

    let show_button = if *rank == "owner" {
        html! {
            <button type="button" onclick={ondelete}
                class="rounded-md px-2 border border-rose-500 text-rose-500 hover:bg-rose-500 hover:text-white active:bg-rose-300">
                {"Delete"}
            </button>
        }
    } else {
        html! {
            <button type="button" onclick={onleave}
                class="rounded-md px-2 border border-rose-500 text-rose-500 hover:bg-rose-500 hover:text-white active:bg-rose-300">
                {"Leave"}
            </button>
        }
    };

    html! {
        <>
        <div class="px-3 my-3 w-full">
            <div class="relative">
                <SearchBox searching={searching.clone()} onenter={onenter.clone()}
                    placeholder={"Search members..."}/>
            </div>
        </div>

        <div class="px-5 py-1 flex justify-between items-center">
            <p class="text-slate-500 font-bold text-xs">
                {"MEMBERS"}
                <span class="ml-2 text-sky-700">{members.len()}</span>
            </p>
            {show_switch}
        </div>

        {
            if *open_adding {
                html! {
                    <AddMember chat={props.chat.clone()} trigger={props.trigger.clone()}
                        curr_room_id={props.curr_room_id.clone()} />
                }
            } else {
                html! {
                    <>
                    <ul class="grow px-2 py-1 overflow-y-scroll hover:scrollbar no-scrollbar">
                        {show_member_list}
                    </ul>

                    <div class="shrink-0 h-12 border-t flex items-center justify-center">
                        {show_button}
                    </div>
                    </>
                }
            }
        }
        </>
    }
}

// ========================// MemberEntry //======================== //

#[derive(PartialEq, Properties)]
struct MemberEntryProps {
    member: MemberItem,
    nav_room: Callback<i64>,
    add_friend: Callback<i64>,
}

#[function_component]
fn MemberEntry(props: &MemberEntryProps) -> Html {
    let show_rank = {
        let color = match props.member.rank.as_str() {
            "owner" => "text-orange-600",
            "manager" => "text-blue-600",
            _ => "text-slate-600",
        };
        html! {
            <div class={classes!("p-1", "group-hover:hidden", color)}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
                    class="w-4 h-4">
                    <path fill-rule="evenodd"
                        d="M7.5 6a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM3.751 20.105a8.25 8.25 0 0116.498 0 .75.75 0 01-.437.695A18.683 18.683 0 0112 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 01-.437-.695z"
                        clip-rule="evenodd" />
                </svg>
            </div>
        }
    };

    let onadding = {
        let add_friend = props.add_friend.clone();
        let friend_id = props.member.id;
        move |_| add_friend.emit(friend_id)
    };

    let show_ops = match props.member.relation {
        Relation::Friend(room_id) => {
            let onchat = {
                let nav_room = props.nav_room.clone();
                move |_| nav_room.emit(room_id)
            };
            html! {
                <div onclick={onchat}
                    class="hidden group-hover:block p-1 cursor-pointer text-slate-500 hover:text-sky-700 active:text-sky-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M8.625 12a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H8.25m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H12m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 01-2.555-.337A5.972 5.972 0 015.41 20.97a5.969 5.969 0 01-.474-.065 4.48 4.48 0 00.978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25z" />
                    </svg>
                </div>
            }
        }
        Relation::Stranger => {
            html! {
                <div onclick={onadding}
                    class="hidden group-hover:block p-1 cursor-pointer text-slate-500 hover:text-sky-700  active:text-sky-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z" />
                    </svg>
                </div>
            }
        }
        _ => html! {
            <div class="hidden group-hover:block p-1 text-slate-500">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                    stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z" />
                </svg>
            </div>
        },
    };

    html! {
        <li class="group flex items-center space-x-2 p-2 rounded-md cursor-default hover:shadow-sm hover:shadow-slate-400 ">
            <div class="shrink-0 rounded-full">
                <Avatar image={props.member.avatar.clone()} classes={"h-7 w-7"} />
            </div>
            <span class="grow truncate text-slate-500 text-sm font-semibold">
                {props.member.name.clone()}
            </span>

            {show_rank}
            {show_ops}
        </li>
    }
}

// ========================// AddMember //======================== //

#[derive(PartialEq, Properties)]
struct AddMemberProps {
    chat: ChatState,
    trigger: MemberListTrigger,
    curr_room_id: UseStateHandle<i64>,
}

#[function_component]
fn AddMember(props: &AddMemberProps) -> Html {
    // show friends who can be added
    let friends = use_memo(|_| props.chat.get_accepted_friends(), props.trigger.clone());

    let selected: UseStateHandle<Vec<i64>> = use_state(|| Vec::new());
    let onselect = {
        let selected = selected.clone();
        Callback::from(move |id: i64| {
            if let Some(idx) = (*selected).iter().position(|&x| x == id) {
                let mut new = (*selected).clone();
                new.swap_remove(idx);
                selected.set(new)
            } else {
                let mut new = (*selected).clone();
                new.push(id);
                selected.set(new);
            }
        })
    };

    let onsubmit = {
        let chat = props.chat.clone();
        let selected = selected.clone();
        let room_id = *props.curr_room_id;
        move |_| {
            let member_ids = (*selected).clone();
            if member_ids.is_empty() {
                return;
            }
            chat.add_members(room_id, member_ids);
        }
    };

    html! {
        <>
        <ul class="grow px-2 py-1 overflow-y-scroll hover:scrollbar no-scrollbar">
            {
                friends.iter().map(|item| {
                    html! {
                        <FriendEntry key={item.id} friend={item.clone()} selected={selected.clone()}
                            onselect={onselect.clone()} />
                    }
                }).collect::<Html>()
            }
        </ul>

        <div class="shrink-0 h-12 border-t flex items-center justify-center">
            <button type="button" onclick={onsubmit}
                class="rounded-md px-4 py-0.5 border border-sky-600 bg-sky-600 text-slate-50 hover:bg-sky-700 hover:text-white active:bg-sky-600">
                {"Submit"}
            </button>
        </div>
        </>
    }
}

// ========================// FriendEntry //======================== //

#[derive(PartialEq, Properties)]
struct FriendEntryProps {
    friend: FriendItem,
    selected: UseStateHandle<Vec<i64>>,
    onselect: Callback<i64>,
}

#[function_component]
fn FriendEntry(props: &FriendEntryProps) -> Html {
    let id = AttrValue::from(format!("{}", props.friend.id));

    let checked = (*props.selected).contains(&props.friend.id);
    let onclick = {
        let onselect = props.onselect.clone();
        let friend_id = props.friend.id;
        move |_| onselect.emit(friend_id)
    };

    html! {
        <li
            class="group flex items-center space-x-2 p-2 rounded-md cursor-default hover:shadow-sm hover:shadow-slate-400 ">
            <input id={id.clone()} type="checkbox" {checked} {onclick} class="w-4 h-4 mr-2" />
            <div class="shrink-0 rounded-full">
                <Avatar image={props.friend.avatar.clone()} classes={"h-7 w-7"} />
            </div>
            <label for={id} class="grow truncate text-slate-500 text-sm font-semibold">
                {props.friend.nickname.clone()}
            </label>
        </li>
    }
}
