use crate::{
    api::{auth::AuthApi, config::ApiConfig},
    hooks::use_local_storage,
    pages::route::Route,
    states::auth::{AuthAction, AuthContext},
    PERSIST,
};
use shared::auth::LogoutResponse;
use std::rc::Rc;
use yew::{hook, use_context};
use yew_router::prelude::use_navigator;

// ========================// use_logout //======================== //

#[hook]
pub fn use_logout() -> Rc<dyn Fn()> {
    let auth = use_context::<AuthContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let persist = use_local_storage::<bool>(PERSIST);

    let logout = Rc::new(move || {
        let api = AuthApi::new(ApiConfig::Logout);
        let auth = auth.clone();
        let navigator = navigator.clone();
        let persist = persist.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(_) = api.send::<(), LogoutResponse>(None).await {
                persist.set(false);
                auth.dispatch(AuthAction::Clear);
                navigator.replace(&Route::Login);
            }
        })
    });

    logout
}
