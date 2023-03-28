use crate::{pages::route::Route, states::auth::AuthContext};
use yew::{function_component, html, use_context, Children, Html, Properties};
use yew_router::prelude::Redirect;

// ========================// AuthGuard //======================== //

#[derive(PartialEq, Properties)]
pub struct AuthGuardProps {
    pub children: Children,
}

#[function_component]
pub fn AuthGuard(props: &AuthGuardProps) -> Html {
    let auth = use_context::<AuthContext>().expect("no auth context");

    if auth.authorized {
        html! {<>{ props.children.clone() }</>}
    } else {
        html! {<Redirect<Route> to={Route::Login}/>}
    }
}
