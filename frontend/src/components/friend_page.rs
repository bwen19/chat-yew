use super::{common::Avatar, friend_list::FriendList, new_friend::NewFriend};
use crate::{
    pages::ChatPage,
    states::{
        chat::ChatState,
        friend::{CurrFriendTrigger, FriendPageTrigger},
        types::Relation,
    },
};
use yew::{
    function_component, html, use_memo, use_state_eq, Callback, Html, Properties, UseStateHandle,
};

// ========================// FriendPage //======================== //

#[derive(PartialEq, Properties)]
pub struct FriendPageProps {
    pub chat: ChatState,
    pub trigger: FriendPageTrigger,
    pub nav_page: Callback<ChatPage>,
}

#[function_component]
pub fn FriendPage(props: &FriendPageProps) -> Html {
    let curr_friend_id = use_state_eq(|| props.chat.curr_friend());

    let nav_friend = {
        let curr_friend_id = curr_friend_id.clone();
        let chat = props.chat.clone();

        Callback::from(move |friend_id: i64| {
            chat.set_curr_friend(friend_id);
            curr_friend_id.set(friend_id);
        })
    };

    html! {
        <>
        // middle friend list
        <div class="shrink-0 h-full w-64 pt-3 border-r bg-slate-200 flex flex-col">
            <FriendList chat={props.chat.clone()} trigger={props.trigger.to_friends()}
                curr_friend_id={curr_friend_id.clone()} nav_friend={nav_friend.clone()} />
        </div>

        // right detail box
        {
            if *curr_friend_id == -1 {
                html! {<NewFriend chat={props.chat.clone()} trigger={props.trigger.to_new()}
                    nav_page={props.nav_page.clone()} />}
            } else {
                html! {
                    <div class="grow h-full bg-slate-100">
                        <CurrFriend chat={props.chat.clone()}
                            trigger={props.trigger.to_curr()}
                            nav_page={props.nav_page.clone()}
                            curr_friend_id={curr_friend_id.clone()} />
                    </div>
                }
            }
        }
        </>
    }
}

// ========================// CurrFriend //======================== //

#[derive(PartialEq, Properties)]
struct CurrFriendProps {
    chat: ChatState,
    trigger: CurrFriendTrigger,
    curr_friend_id: UseStateHandle<i64>,
    nav_page: Callback<ChatPage>,
}

#[function_component]
fn CurrFriend(props: &CurrFriendProps) -> Html {
    let friend = use_memo(
        |(friend_id, _)| props.chat.get_curr_friend(*friend_id),
        (*props.curr_friend_id.clone(), props.trigger.clone()),
    );

    if let Some(ref friend) = *friend {
        let onchat = {
            let nav_page = props.nav_page.clone();
            let chat = props.chat.clone();
            let relation = friend.relation.clone();
            move |_| {
                if let Relation::Friend(room_id) = relation {
                    chat.set_curr_room(room_id);
                    nav_page.emit(ChatPage::Room);
                }
            }
        };

        let ondelete = {
            let chat = props.chat.clone();
            let friend_id = friend.id;
            move |_| chat.delete_friend(friend_id)
        };

        html! {
            <>
            <div class="shrink-0 h-14 border-b px-4 flex items-center justify-center">
                <p class="text-slate-600 font-semibold">{"Information"}</p>
            </div>
            <div class="pt-16 flex flex-col items-center space-y-5">
                <div class="shrink-0 rounded-full">
                    <Avatar classes="h-24 w-24" image={friend.avatar.clone()} />
                </div>
                <p class="text-slate-700 text-lg font-semibold">
                    {friend.username.clone()}
                </p>
                <div>
                    <p class="text-slate-500">
                        {"Nick:"}
                        <span class="ml-2">{friend.nickname.clone()}</span>
                    </p>

                </div>
                <button type="button" onclick={onchat}
                    class="rounded-md px-6 py-1 border border-sky-600 text-sky-600 hover:bg-sky-600 hover:text-white active:bg-sky-400">
                    {"Chat"}
                </button>
                <button type="button" onclick={ondelete}
                    class="rounded-md px-6 py-1 border border-rose-500 text-rose-500 hover:bg-rose-500 hover:text-white active:bg-rose-300">
                    {"Delete"}
                </button>
            </div>
            </>
        }
    } else {
        html! {
            <div class="grow h-full py-1 bg-slate-100 flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.0"
                    stroke="currentColor" class="w-24 h-24 stroke-slate-300">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
                </svg>
            </div>
        }
    }
}
