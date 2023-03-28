use frontend::components::toast::Toast;
use frontend::pages::route::{switch, Route};
use frontend::states::{
    auth::{AuthContext, AuthState},
    toast::{ToastContext, ToastState},
};
use yew::{function_component, html, use_reducer, ContextProvider, Html};
use yew_router::{BrowserRouter, Switch};

// ========================// App //======================== //

#[function_component]
fn App() -> Html {
    let auth = use_reducer(AuthState::default);
    let toast = use_reducer(ToastState::default);

    html! {
        <BrowserRouter>
            <ContextProvider<AuthContext> context={auth}>
                <ContextProvider<ToastContext> context={toast}>
                    <Switch<Route> render={switch}/>
                    <Toast />
                </ContextProvider<ToastContext>>
            </ContextProvider<AuthContext>>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
