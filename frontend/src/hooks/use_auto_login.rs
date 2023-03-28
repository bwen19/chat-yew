use crate::{
    api::{auth::AuthApi, config::ApiConfig},
    hooks::use_local_storage,
    pages::route::Route,
    states::auth::{AuthAction, AuthContext},
    PERSIST,
};
use shared::auth::LoginResponse;
use std::rc::Rc;
use yew::{hook, use_context, use_state, UseStateHandle};
use yew_router::prelude::use_navigator;

use super::UseLocalStorageHandle;

// ========================// UseAutoLoginHandle //======================== //

pub struct UseAutoLoginHandle {
    loading: UseStateHandle<bool>,
    persist: UseLocalStorageHandle<bool>,
    auto_login: Rc<dyn Fn()>,
}

impl Clone for UseAutoLoginHandle {
    fn clone(&self) -> Self {
        Self {
            loading: self.loading.clone(),
            persist: self.persist.clone(),
            auto_login: self.auto_login.clone(),
        }
    }
}

impl UseAutoLoginHandle {
    pub fn loading(&self) -> bool {
        *self.loading
    }

    pub fn persist(&self) -> UseLocalStorageHandle<bool> {
        self.persist.clone()
    }

    pub fn run(&self) {
        (*self.auto_login)();
    }
}

// ========================// use_auto_login //======================== //

#[hook]
pub fn use_auto_login() -> UseAutoLoginHandle {
    let auth = use_context::<AuthContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let persist = use_local_storage::<bool>(PERSIST);
    let loading = use_state(|| false);

    let auto_login = {
        let loading = loading.clone();
        let persist = persist.clone();

        Rc::new(move || {
            if *persist {
                let auth = auth.clone();
                let navigator = navigator.clone();
                let persist = persist.clone();
                let loading = loading.clone();
                let api = AuthApi::new(ApiConfig::AutoLogin);

                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    if let Ok(data) = api.send::<(), LoginResponse>(None).await {
                        auth.dispatch(AuthAction::Set(data));
                        navigator.push(&Route::Chat);
                    } else {
                        persist.set(false);
                    }
                    loading.set(false);
                });
            }
        })
    };

    UseAutoLoginHandle {
        loading,
        persist,
        auto_login,
    }
}
