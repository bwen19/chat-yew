use super::common::Avatar;
use crate::states::{chat::ChatState, room::NewRoomTrigger, types::FriendItem};
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_memo, use_node_ref, use_state, AttrValue, Callback, Html,
    Properties, UseStateHandle,
};

// ========================// NewRoom //======================== //

#[derive(PartialEq, Properties)]
pub struct NewRoomProps {
    pub trigger: NewRoomTrigger,
    pub chat: ChatState,
}

#[function_component]
pub fn NewRoom(props: &NewRoomProps) -> Html {
    let friends = use_memo(|_| props.chat.get_accepted_friends(), props.trigger.clone());

    let input_node_ref = use_node_ref();
    let input_value = use_state(|| String::default());
    let oninput = {
        let input_value = input_value.clone();
        let input_node_ref = input_node_ref.clone();

        move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                input_value.set(input.value());
            }
        }
    };

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
        let input_value = input_value.clone();
        let selected = selected.clone();
        move |_| {
            let name = (*input_value).clone();
            let member_ids = (*selected).clone();
            if name.is_empty() || member_ids.is_empty() {
                return;
            }
            chat.create_room(name, member_ids);
        }
    };

    html! {
        <div class="grow h-full py-1 bg-slate-100 flex flex-col items-center">
            <div class="shrink-0 h-14 w-full border-b px-4 flex items-center justify-center">
                <p class="text-slate-700 font-semibold">{"New room"}</p>
            </div>

            <div class="shrink-0 h-20 w-80 flex items-center justify-between">
                <label for="room-name" class="block mr-3 min-w-0 shrink-0 text-slate-500 font-semibold">{"Name:"}</label>
                <input id="room-name" type="text" ref={input_node_ref} value={(*input_value).clone()} {oninput}
                    class="appearance-none w-full py-1 px-2 rounded-md bg-white text-slate-700 leading-tight border-2 border-slate-300 focus:outline-none focus:border-sky-600" />
            </div>

            {
                if friends.is_empty() {
                    html! {
                        <div class="w-80 p-1 font-semibold text-slate-400 text-center">
                            {"You have no friends to create a room!"}
                        </div>
                    }
                } else {
                    html! {
                        <>
                        <div class="w-80 p-1 overflow-y-scroll hover:scrollbar no-scrollbar">
                            <ul class="py-2 rounded-md shadow-sm bg-white">
                                {
                                    friends.iter().map(|item| {
                                        html! {
                                            <FriendEntry key={item.id} friend={item.clone()} selected={selected.clone()}
                                                onselect={onselect.clone()} />
                                        }
                                    }).collect::<Html>()}
                            </ul>
                        </div>
                        <div class="shrink-0 h-16 flex items-center justify-center">
                            <button type="button" onclick={onsubmit}
                                class="rounded-md px-6 py-1 border border-sky-600 text-slate-50 bg-sky-600 hover:bg-sky-700 active:bg-sky-600">
                                {"Submit"}
                            </button>
                        </div>
                        </>
                    }
                }
            }

        </div>
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
    let id = AttrValue::from(format!("id-{}", props.friend.id));

    let checked = (*props.selected).contains(&props.friend.id);
    let onclick = {
        let onselect = props.onselect.clone();
        let friend_id = props.friend.id;
        move |_| onselect.emit(friend_id)
    };

    html! {
        <li class="px-4 py-3 flex items-center space-x-3 hover:bg-slate-200">
            <input id={id.clone()} type="checkbox" {checked} {onclick} class="w-4 h-4 mr-2" />
            <label for={id.clone()} class="pl-2 shrink-0 text-slate-600">
                <Avatar image={props.friend.avatar.clone()} classes={"h-8 w-8"} />
            </label>
            <label for={id} class="min-w-0 flex-1 text-slate-600">
                {props.friend.nickname.clone()}
            </label>
        </li>
    }
}
