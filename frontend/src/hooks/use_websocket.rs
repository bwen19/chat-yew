use crate::{
    hooks::use_logout,
    states::types::ConvertToMessage,
    states::{
        auth::AuthContext,
        trigger::{Trigger, TriggerState},
        websocket::WebSocketState,
    },
};
use futures::{channel::mpsc, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message, WebSocketError};
use shared::event::{ClientEvent, ServerEvent};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use yew::{hook, use_context, use_effect_with_deps, use_mut_ref, use_reducer, use_state};

const RECONNECT_LIMIT: u8 = 3;

// ========================// UseWebSocketHandle //======================== //

pub struct UseWebSocketHandle {
    pub inner: Rc<RefCell<WebSocketState>>,
    pub trigger: Trigger,
}

impl UseWebSocketHandle {
    pub fn set_tx(&self, tx: Option<mpsc::Sender<Message>>) {
        self.inner.borrow_mut().tx = tx;
    }

    pub fn handle_event(&mut self, event: ServerEvent) -> Result<(), String> {
        let mut chat = self.inner.borrow_mut();
        chat.handle_event(event, self.trigger.clone())
    }
}

impl Clone for UseWebSocketHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            trigger: self.trigger.clone(),
        }
    }
}

// ========================// use_websocket //======================== //

#[hook]
pub fn use_websocket() -> UseWebSocketHandle {
    let logout = use_logout();
    let auth = use_context::<AuthContext>().unwrap();

    let inner = use_mut_ref(|| WebSocketState::new(auth.user.id));
    let trigger = use_reducer(TriggerState::default);

    let websocket = UseWebSocketHandle { inner, trigger };

    // connection count
    let count = use_state(|| 1_u8);

    {
        let count_ref = count.clone();
        let mut websocket = websocket.clone();

        use_effect_with_deps(
            move |&count| {
                if count >= RECONNECT_LIMIT {
                    logout();
                    return;
                }

                // let location = web_sys::window().unwrap_throw().location();
                // let host = location.host().unwrap_throw();
                // let protocol = location.protocol().unwrap_throw();
                // let ws_protocol = match protocol.as_str() {
                //     "https:" => "wss:",
                //     _ => "ws:",
                // };
                // let ws_addr = format!("{}//{}/ws", ws_protocol, host);
                // let ws = WebSocket::open(&ws_addr).unwrap_throw();
                let conn = WebSocket::open("ws://127.0.0.1:8080/ws").unwrap_throw();

                let (mut sender, mut receiver) = conn.split();
                let (tx, mut rx) = mpsc::channel::<Message>(256);
                websocket.set_tx(Some(tx));

                // task for sending message to server
                spawn_local(async move {
                    let msg = ClientEvent::Initialization.to_msg();
                    sender.send(msg).await.unwrap_throw();

                    while let Some(msg) = rx.next().await {
                        sender.send(msg).await.unwrap_throw();
                    }
                });

                // task for receiving message and processing events
                spawn_local(async move {
                    while let Some(msg) = receiver.next().await {
                        match msg {
                            Ok(Message::Text(msg)) => {
                                gloo_console::log!(msg);
                            }
                            Ok(Message::Bytes(b)) => {
                                if let Ok(event) = serde_json::from_slice::<ServerEvent>(&b) {
                                    if let Err(s) = websocket.handle_event(event) {
                                        gloo_console::log!(s);
                                        count_ref.set(RECONNECT_LIMIT);
                                        break;
                                    }
                                }
                            }
                            Err(e) => match e {
                                WebSocketError::ConnectionError => {
                                    gloo_console::error!("Error on connection");
                                    count_ref.set(*count_ref + 1);
                                    break;
                                }
                                WebSocketError::ConnectionClose(e) => {
                                    gloo_console::error!(
                                        "The connection has been closed :",
                                        e.code
                                    );
                                    count_ref.set(RECONNECT_LIMIT);
                                    break;
                                }
                                WebSocketError::MessageSendError(e) => {
                                    gloo_console::error!(
                                        "Error while sending message",
                                        e.to_string()
                                    );
                                    count_ref.set(RECONNECT_LIMIT);
                                    break;
                                }
                                _ => gloo_console::error!(
                                    "Unexpected error while communicating with distant ws"
                                ),
                            },
                        }
                    }
                    websocket.set_tx(None);
                });
            },
            *count,
        );
    }

    websocket
}
