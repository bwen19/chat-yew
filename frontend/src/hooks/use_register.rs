use crate::{
    api::{auth::AuthApi, config::ApiConfig, ApiError},
    pages::route::Route,
    states::toast::{ToastAction, ToastContext},
};
use shared::auth::{RegisterRequest, RegisterResponse};
use std::rc::Rc;
use yew::{hook, use_context, use_state, AttrValue, UseStateHandle};
use yew_router::prelude::use_navigator;

// ========================// UseRegisterHandle //======================== //

pub struct UseRegisterHandle {
    loading: UseStateHandle<bool>,
    register: Rc<dyn Fn(RegisterRequest)>,
}

impl Clone for UseRegisterHandle {
    fn clone(&self) -> Self {
        Self {
            loading: self.loading.clone(),
            register: self.register.clone(),
        }
    }
}

impl UseRegisterHandle {
    pub fn loading(&self) -> bool {
        *self.loading
    }

    pub fn run(&self, req: RegisterRequest) {
        (*self.register)(req);
    }
}

// ========================// use_register //======================== //

#[hook]
pub fn use_register() -> UseRegisterHandle {
    let toast = use_context::<ToastContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let loading = use_state(|| false);

    let register = {
        let loading = loading.clone();

        Rc::new(move |payload: RegisterRequest| {
            let toast = toast.clone();
            let navigator = navigator.clone();
            let loading = loading.clone();
            let api = AuthApi::new(ApiConfig::Register);

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match api
                    .send::<RegisterRequest, RegisterResponse>(Some(&payload))
                    .await
                {
                    Ok(data) => {
                        let msg = AttrValue::from(format!(
                            "Registration successfully, {}",
                            data.user.username
                        ));
                        toast.dispatch(ToastAction::Info(msg));
                        navigator.push(&Route::Login);
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

    UseRegisterHandle { loading, register }
}
