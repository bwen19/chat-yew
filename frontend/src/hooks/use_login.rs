use crate::{
    api::{auth::AuthApi, config::ApiConfig, ApiError},
    pages::route::Route,
    states::{
        auth::{AuthAction, AuthContext},
        toast::{ToastAction, ToastContext},
    },
};
use shared::auth::{LoginRequest, LoginResponse};
use std::rc::Rc;
use yew::{hook, use_context, use_state, UseStateHandle};
use yew_router::prelude::use_navigator;

// ========================// UseLoginHandle //======================== //

pub struct UseLoginHandle {
    loading: UseStateHandle<bool>,
    login: Rc<dyn Fn(LoginRequest)>,
}

impl Clone for UseLoginHandle {
    fn clone(&self) -> Self {
        Self {
            loading: self.loading.clone(),
            login: self.login.clone(),
        }
    }
}

impl UseLoginHandle {
    pub fn loading(&self) -> bool {
        *self.loading
    }

    pub fn run(&self, req: LoginRequest) {
        (*self.login)(req);
    }
}

// ========================// use_login //======================== //

#[hook]
pub fn use_login() -> UseLoginHandle {
    let auth = use_context::<AuthContext>().unwrap();
    let toast = use_context::<ToastContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let loading = use_state(|| false);

    let login = {
        let loading = loading.clone();

        Rc::new(move |payload: LoginRequest| {
            let auth = auth.clone();
            let toast = toast.clone();
            let navigator = navigator.clone();
            let loading = loading.clone();
            let api = AuthApi::new(ApiConfig::Login);

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match api
                    .send::<LoginRequest, LoginResponse>(Some(&payload))
                    .await
                {
                    Ok(data) => {
                        auth.dispatch(AuthAction::Set(data));
                        navigator.push(&Route::Chat);
                    }
                    Err(e) => {
                        if let ApiError::Toast(msg) = e {
                            toast.dispatch(ToastAction::Error(msg.into()));
                        }
                    }
                }
                loading.set(false);
            });
        })
    };

    UseLoginHandle { loading, login }
}
