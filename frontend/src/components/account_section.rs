use super::common::Avatar;
use crate::{
    hooks::{use_click_away, use_logout},
    states::auth::AuthContext,
};
use yew::{function_component, html, use_context, use_node_ref, use_state, Callback, Html};

// ========================// AccountSection //======================== //

#[function_component]
pub fn AccountSection() -> Html {
    let auth = use_context::<AuthContext>().unwrap();
    let logout = use_logout();

    let open = use_state(|| false);

    let onclick = {
        let open = open.clone();
        move |_| open.set(!*open)
    };

    let node_ref = use_node_ref();
    let onclose = {
        let open = open.clone();
        Callback::from(move |_| open.set(false))
    };
    use_click_away(node_ref.clone(), onclose);

    let onlogout = move |_| logout();

    html! {
        <>
        <div {onclick} class="rounded-full cursor-pointer hover:ring-2 hover:ring-slate-50">
            <Avatar image={auth.user.avatar.clone()} classes="h-10 w-10" />
        </div>

        <div ref={node_ref} hidden={!*open} class="absolute bottom-4 left-14 z-10 w-44 divide-y divide-slate-200 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5">
            <div class="py-1.5 text-center text-slate-400">
                {auth.user.username.clone()}
            </div>
            <div class="py-1">
                <div class="px-4 py-2 cursor-pointer text-sm text-slate-600 hover:bg-sky-600 hover:text-slate-50">
                    {"Upadte Profile"}
                </div>
                <div class="px-4 py-2 cursor-pointer text-sm text-slate-600 hover:bg-sky-600 hover:text-slate-50">
                    {"Change Password"}
                </div>
            </div>
            <div class="py-1">
                <div onclick={onlogout} class="px-4 py-2 cursor-pointer text-sm text-slate-600 hover:bg-sky-600 hover:text-slate-50">
                    {"Logout"}
                </div>
            </div>
        </div>
        </>
    }
}
