use super::common::Avatar;
use crate::states::{chat::ChatState, room::MessageListTrigger, types::MessageItem};
use shared::message::NewMessageRequest;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::{
    function_component, html, use_effect, use_memo, use_node_ref, use_state, Html, Properties,
    UseStateHandle,
};

// ========================// MessageList //======================== //

#[derive(PartialEq, Properties)]
pub struct MessageListProps {
    pub chat: ChatState,
    pub trigger: MessageListTrigger,
    pub curr_room_id: UseStateHandle<i64>,
}

#[function_component]
pub fn MessageList(props: &MessageListProps) -> Html {
    let messages = use_memo(
        |(_, room_id)| props.chat.get_messages(*room_id),
        (props.trigger.clone(), *props.curr_room_id),
    );

    let show_messages = messages
        .iter()
        .map(|message| {
            html! {
                <MessageEntry key={message.key().as_str()} message={message.clone()} />
            }
        })
        .collect::<Html>();

    let box_node_ref = use_node_ref();
    {
        let box_node_ref = box_node_ref.clone();
        use_effect(move || {
            if let Some(ele) = box_node_ref.cast::<HtmlElement>() {
                ele.scroll_to_with_x_and_y(0.0, ele.scroll_height() as f64);
            }
        });
    }

    html! {
        <>
        // chat content
        <div ref={box_node_ref} class="grow p-3 flex flex-col space-y-5 overflow-y-scroll hover:scrollbar no-scrollbar">
            {show_messages}
        </div>

        // chat sender
        <SendMessage chat={props.chat.clone()} trigger={props.trigger.clone()}
            curr_room_id={props.curr_room_id.clone()} />

        </>
    }
}

// ========================// MessageEntry //======================== //

#[derive(PartialEq, Properties)]
struct MessageEntryProps {
    message: MessageItem,
}

#[function_component]
fn MessageEntry(props: &MessageEntryProps) -> Html {
    match props.message {
        MessageItem::TimeDivider(ref date) => {
            // divider for time
            html! {
                <div class="text-center pt-7 pb-2">
                    <hr class="-mb-3.5" />
                    <span class="px-3 py-0.5 -mt-3.5 rounded-lg bg-white text-xs text-slate-400">
                        {date.clone()}
                    </span>
                </div>
            }
        }
        MessageItem::Outgoing(ref item) => {
            // message send by yourself
            html! {
                <div class="flex flex-row-reverse">
                    <div class="w-5/6 flex flex-row-reverse items-start space-x-2 space-x-reverse">
                        <div class="shrink-0 rounded-full">
                            <Avatar image={item.avatar.clone()} classes={"h-8 w-8"} />
                        </div>
                        <div class="grow flex flex-col items-end">
                            <p class="w-fit mb-2 font-semibold text-sm text-slate-700">
                                {item.name.clone()}
                                <span class="ml-2 text-slate-400 text-xs">
                                    {item.send_at.clone()}
                                </span>
                            </p>
                            <div class="w-fit px-3 py-2 bg-sky-600 shadow-md rounded-b-lg rounded-tl-lg text-sm text-white">
                                {item.content.clone()}
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
        MessageItem::Incoming(ref item) => {
            // messages from others
            html! {
                <div class="flex">
                    <div class="w-5/6 flex items-start space-x-2">
                        <div class="shrink-0 rounded-full">
                            <Avatar image={item.avatar.clone()} classes={"h-8 w-8"} />
                        </div>
                        <div class="grow">
                            <p class="w-fit mb-2 font-semibold text-sm text-slate-700">
                                {item.name.clone()}
                                <span class="ml-2 text-slate-400 text-xs">
                                    {item.send_at.clone()}
                                </span>
                            </p>
                            <div class="w-fit px-3 py-2 bg-white shadow-sm rounded-b-lg rounded-tr-lg text-sm text-slate-700">
                                {item.content.clone()}
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}

// ========================// SendMessage //======================== //

#[derive(PartialEq, Properties)]
struct SendMessageListProps {
    chat: ChatState,
    trigger: MessageListTrigger,
    curr_room_id: UseStateHandle<i64>,
}

#[function_component]
fn SendMessage(props: &SendMessageListProps) -> Html {
    let input_node_ref = use_node_ref();
    let input_value = use_state(String::default);

    let oninput = {
        let input_value = input_value.clone();
        let input_node_ref = input_node_ref.clone();

        move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                input_value.set(input.value());
            }
        }
    };

    let onsend = {
        let chat = props.chat.clone();
        let curr_room_id = props.curr_room_id.clone();
        let input_value = input_value.clone();

        move |_| {
            let msg = NewMessageRequest {
                room_id: *curr_room_id,
                content: (*input_value).clone(),
                kind: "text".to_owned(),
            };
            chat.send_message(msg);
            input_value.set(String::default());
        }
    };

    html! {
        <div class="shrink-0 h-16 border-t px-4 flex justify-center items-center">
            <div class="flex justify-between w-full items-center bg-white rounded-lg">
                <div class="mx-2 rounded-full p-1 cursor-pointer text-slate-400 hover:text-sky-600 active:text-slate-400">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M18.375 12.739l-7.693 7.693a4.5 4.5 0 01-6.364-6.364l10.94-10.94A3 3 0 1119.5 7.372L8.552 18.32m.009-.01l-.01.01m5.699-9.941l-7.81 7.81a1.5 1.5 0 002.112 2.13" />
                    </svg>
                </div>
                <input type="text" spellcheck="false" ref={input_node_ref} {oninput} value={(*input_value).clone()}
                    class="w-full py-1 bg-transparent outline-none placeholder:text-slate-300 text-slate-700"
                    placeholder="Type your message here..." />
                <div class="mx-3 flex items-center space-x-3">
                    <div class="rounded-full p-1 cursor-pointer text-slate-400 hover:text-sky-600 active:text-slate-400">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                            stroke="currentColor" class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M15.182 15.182a4.5 4.5 0 01-6.364 0M21 12a9 9 0 11-18 0 9 9 0 0118 0zM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75 9.168 9 9.375 9s.375.336.375.75zm-.375 0h.008v.015h-.008V9.75zm5.625 0c0 .414-.168.75-.375.75s-.375-.336-.375-.75.168-.75.375-.75.375.336.375.75zm-.375 0h.008v.015h-.008V9.75z" />
                        </svg>
                    </div>

                    <div onclick={onsend} class="rounded-full p-2 cursor-pointer bg-sky-600 hover:bg-sky-800 active:bg-sky-600">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
                            class="w-5 h-5 fill-white -rotate-45">
                            <path d="M3.478 2.405a.75.75 0 00-.926.94l2.432 7.905H13.5a.75.75 0 010 1.5H4.984l-2.432 7.905a.75.75 0 00.926.94 60.519 60.519 0 0018.445-8.986.75.75 0 000-1.218A60.517 60.517 0 003.478 2.405z" />
                        </svg>
                    </div>

                </div>
            </div>
        </div>
    }
}
