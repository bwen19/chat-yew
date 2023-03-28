use crate::{
    api::{config::ApiConfig, private::PrivateApi, ApiError},
    states::{
        auth::AuthContext,
        toast::{ToastAction, ToastContext},
    },
};
use serde::{de::DeserializeOwned, Serialize};
use std::{ops::Deref, rc::Rc};
use yew::{hook, use_context, use_state, UseStateHandle};

// ========================// UseRequestHandle //======================== //

pub struct UseRequestHandle<T, D> {
    loading: UseStateHandle<bool>,
    data: UseStateHandle<Option<D>>,
    send: Rc<dyn Fn(T)>,
}

impl<T, D> Clone for UseRequestHandle<T, D> {
    fn clone(&self) -> Self {
        Self {
            loading: self.loading.clone(),
            data: self.data.clone(),
            send: self.send.clone(),
        }
    }
}

impl<T, D> PartialEq for UseRequestHandle<T, D>
where
    D: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.data == *other.data
    }
}

impl<T, D> Deref for UseRequestHandle<T, D> {
    type Target = Option<D>;

    fn deref(&self) -> &Self::Target {
        &(*self.data)
    }
}

impl<T, D> UseRequestHandle<T, D> {
    pub fn loading(&self) -> bool {
        *self.loading
    }

    pub fn send(&self, req: T) {
        (*self.send)(req);
    }
}

// ========================// use_request //======================== //

#[hook]
pub fn use_request<T, D>(api_config: ApiConfig) -> UseRequestHandle<T, D>
where
    T: Serialize + 'static,
    D: DeserializeOwned + 'static,
{
    let auth = use_context::<AuthContext>().unwrap();
    let toast = use_context::<ToastContext>().unwrap();
    let loading = use_state(|| false);
    let data = use_state(|| None);

    let send = {
        let loading = loading.clone();
        let data = data.clone();
        let api = PrivateApi::new(api_config, auth);

        Rc::new(move |payload: T| {
            let toast = toast.clone();
            let loading = loading.clone();
            let data = data.clone();
            let api = api.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match api.send::<T, D>(&payload).await {
                    Ok(resp) => data.set(Some(resp)),
                    Err(e) => {
                        if let ApiError::Toast(msg) = e {
                            toast.dispatch(ToastAction::Error(msg.into()));
                        }
                        data.set(None)
                    }
                }
                loading.set(false);
            });
        })
    };

    UseRequestHandle {
        loading,
        data,
        send,
    }
}
