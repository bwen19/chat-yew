use super::ChatPage;
use crate::{
    components::{account_section::AccountSection, friend_page::FriendPage, room_page::RoomPage},
    hooks::use_websocket,
    states::{chat::ChatState, friend::FriendPageTrigger, room::RoomPageTrigger},
};
use yew::{
    classes, function_component, html, use_state_eq, Callback, Html, Properties, UseStateHandle,
};

// ========================// Chat //======================== //

#[function_component]
pub fn Chat() -> Html {
    let ws = use_websocket();
    let curr_page = use_state_eq(|| ChatPage::Room);

    let nav_page = {
        let curr_page = curr_page.clone();
        Callback::from(move |page: ChatPage| {
            curr_page.set(page);
        })
    };

    let show_page = match *curr_page {
        ChatPage::Room => html! {
            <RoomPage chat={ChatState::from_ws(&ws)} trigger={RoomPageTrigger::from_ws(&ws)} />
        },
        ChatPage::Friend => html! {
            <FriendPage chat={ChatState::from_ws(&ws)} trigger={FriendPageTrigger::from_ws(&ws)}
                nav_page={nav_page.clone()}  />
        },
    };

    html! {
        <div class="h-screen w-full min-w-fit p-5 lg:py-20 flex items-center justify-center bg-cover"
            style="background-image: url('/assets/pic/chat-bg.jpg')">
            <div class="h-full max-w-5xl w-full rounded-md shadow-lg overflow-hidden flex items-center justify-center">
                // nav bar
                <div class="shrink-0 h-full w-16 pb-3 flex flex-col bg-slate-800">
                    <div class="grow">
                        <div class="h-16 flex items-center justify-center">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                                stroke="currentColor" class="w-8 h-8 stroke-indigo-400">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M12.75 3.03v.568c0 .334.148.65.405.864l1.068.89c.442.369.535 1.01.216 1.49l-.51.766a2.25 2.25 0 01-1.161.886l-.143.048a1.107 1.107 0 00-.57 1.664c.369.555.169 1.307-.427 1.605L9 13.125l.423 1.059a.956.956 0 01-1.652.928l-.679-.906a1.125 1.125 0 00-1.906.172L4.5 15.75l-.612.153M12.75 3.031a9 9 0 00-8.862 12.872M12.75 3.031a9 9 0 016.69 14.036m0 0l-.177-.529A2.25 2.25 0 0017.128 15H16.5l-.324-.324a1.453 1.453 0 00-2.328.377l-.036.073a1.586 1.586 0 01-.982.816l-.99.282c-.55.157-.894.702-.8 1.267l.073.438c.08.474.49.821.97.821.846 0 1.598.542 1.865 1.345l.215.643m5.276-3.67a9.012 9.012 0 01-5.276 3.67m0 0a9 9 0 01-10.275-4.835M15.75 9c0 .896-.393 1.7-1.016 2.25" />
                            </svg>
                        </div>

                        <div class="flex flex-col items-center space-y-1">
                            <RoomEntry nav_page={nav_page.clone()} curr_page={curr_page.clone()} />
                            <FriendEntry nav_page={nav_page.clone()} curr_page={curr_page.clone()} />
                        </div>
                    </div>

                    <div class="shrink-0 relative h-16 flex justify-center items-center">
                        <AccountSection />
                    </div>
                </div>

                {show_page}
            </div>
        </div>
    }
}

// ========================// RoomEntry //======================== //

#[derive(PartialEq, Properties)]
struct RoomEntryProps {
    nav_page: Callback<ChatPage>,
    curr_page: UseStateHandle<ChatPage>,
}

#[function_component]
fn RoomEntry(props: &RoomEntryProps) -> Html {
    let onclick = {
        let nav_page = props.nav_page.clone();
        move |_| nav_page.emit(ChatPage::Room)
    };

    let cls = if *props.curr_page == ChatPage::Room {
        "p-2 rounded-full cursor-pointer text-slate-300".to_owned()
    } else {
        "p-2 rounded-full cursor-pointer text-slate-500 hover:text-slate-300".to_owned()
    };

    html! {
        <div {onclick} class={classes!(cls)}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M20.25 8.511c.884.284 1.5 1.128 1.5 2.097v4.286c0 1.136-.847 2.1-1.98 2.193-.34.027-.68.052-1.02.072v3.091l-3-3c-1.354 0-2.694-.055-4.02-.163a2.115 2.115 0 01-.825-.242m9.345-8.334a2.126 2.126 0 00-.476-.095 48.64 48.64 0 00-8.048 0c-1.131.094-1.976 1.057-1.976 2.192v4.286c0 .837.46 1.58 1.155 1.951m9.345-8.334V6.637c0-1.621-1.152-3.026-2.76-3.235A48.455 48.455 0 0011.25 3c-2.115 0-4.198.137-6.24.402-1.608.209-2.76 1.614-2.76 3.235v6.226c0 1.621 1.152 3.026 2.76 3.235.577.075 1.157.14 1.74.194V21l4.155-4.155" />
            </svg>
        </div>
    }
}

// ========================// FriendEntry //======================== //

#[derive(PartialEq, Properties)]
struct FriendEntryProps {
    nav_page: Callback<ChatPage>,
    curr_page: UseStateHandle<ChatPage>,
}

#[function_component]
fn FriendEntry(props: &FriendEntryProps) -> Html {
    let onclick = {
        let nav_page = props.nav_page.clone();
        move |_| nav_page.emit(ChatPage::Friend)
    };

    let cls = if *props.curr_page == ChatPage::Friend {
        "p-2 rounded-full cursor-pointer text-slate-300".to_owned()
    } else {
        "p-2 rounded-full cursor-pointer text-slate-500 hover:text-slate-300".to_owned()
    };

    html! {
        <div {onclick} class={classes!(cls)}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
            </svg>
        </div>
    }
}
