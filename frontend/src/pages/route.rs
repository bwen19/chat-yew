use super::{chat::Chat, login::Login, page_not_found::PageNotFound, register::Register};
use crate::components::auth_guard::AuthGuard;
use yew::{html, Html};
use yew_router::Routable;

// ========================// Route //======================== //

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Chat,
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Chat => html! { <AuthGuard><Chat /></AuthGuard> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <PageNotFound /> },
    }
}
