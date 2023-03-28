use crate::hooks::{use_auto_login, use_login, UseLocalStorageHandle};
use shared::auth::LoginRequest;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, NodeRef, Properties};

// ========================// Login //======================== //

#[function_component]
pub fn Login() -> Html {
    let auto_login = use_auto_login();
    let persist = auto_login.persist();

    {
        let auto_login = auto_login.clone();
        use_effect_with_deps(
            move |_| {
                auto_login.run();
            },
            (),
        );
    }

    let loading_mask = if auto_login.loading() {
        html! {
            <div class="absolute w-full h-full flex flex-col items-center justify-center bg-slate-400 opacity-80">
                <svg class="w-8 h-8 mr-2 text-slate-50 animate-spin dark:text-gray-600 fill-sky-700"
                    viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                        fill="currentColor" />
                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                        fill="currentFill" />
                </svg>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class="h-screen w-full px-2 bg-no-repeat bg-center bg-cover flex justify-center items-center"
            style="background-image: url('/assets/pic/login-bg.jpg')">
            <div class="max-w-md lg:max-w-4xl w-full flex rounded-lg overflow-hidden bg-slate-50">
                <div class="basis-1/2 shrink-0 hidden lg:block">
                    <img src="/assets/pic/login-left.png" alt="login" class="h-full object-cover object-center" />
                </div>
                <div class="grow relative h-full flex items-center justify-center">
                    <LoginForm persist={persist} />
                    {loading_mask}
                </div>
            </div>
        </div>
    }
}

// ========================// LoginForm //======================== //

#[derive(Default)]
struct InputForm {
    username: NodeRef,
    password: NodeRef,
}

#[derive(Properties, PartialEq)]
struct LoginFormProps {
    persist: UseLocalStorageHandle<bool>,
}

#[function_component]
fn LoginForm(props: &LoginFormProps) -> Html {
    let login = use_login();
    let input = use_state(|| InputForm::default());

    let onclick = {
        let persist = props.persist.clone();
        move |_| persist.set(!*persist)
    };

    let submit = {
        let input = input.clone();
        let login = login.clone();
        move |_| {
            let username = input
                .username
                .cast::<HtmlInputElement>()
                .map(|x| x.value())
                .unwrap_or("".to_owned());
            let password = input
                .password
                .cast::<HtmlInputElement>()
                .map(|x| x.value())
                .unwrap_or("".to_owned());

            if username.is_empty() || password.is_empty() {
                return;
            }
            let arg = LoginRequest { username, password };
            login.run(arg);
        }
    };

    html! {
        <div class="w-full space-y-8 p-16">
            <h2 class="text-slate-700 text-2xl font-bold text-center">{"Log In"}</h2>
            <form onsubmit={submit} action="javascript:void(0);">
                <label class="block mb-2 text-sm font-semibold text-slate-500"
                    for="inline-username">{"Username"}</label>
                <input id="inline-username" type="text" minlength="2" ref={&input.username}
                    class="appearance-none w-full py-2 px-4 rounded-md bg-white text-slate-700 leading-tight border-2 border-slate-300 focus:outline-none invalid:border-rose-500 focus:border-sky-600 focus:invalid:border-rose-500" />

                <label class="block mt-3 mb-2 text-sm font-semibold text-slate-500"
                    for="inline-password">{"Password"}</label>
                <input id="inline-password" type="password" minlength="6" ref={&input.password}
                    class="appearance-none w-full py-2 px-4 rounded-md bg-white text-slate-700 leading-tight border-2 border-slate-300 focus:outline-none invalid:border-rose-500 focus:border-sky-600 focus:invalid:border-rose-500" />

                // {submit_button}
                <button type="button" disabled={true} hidden={!login.loading()}
                    class="w-full rounded-md mt-7 mb-3 py-2.5 cursor-wait text-sm font-semibold text-slate-400 bg-cyan-700">
                    <svg aria-hidden="true" role="status" class="inline w-4 h-4 mr-3 text-teal-700 animate-spin"
                        viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                            fill="#E5E7EB"/>
                        <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                            fill="currentColor"/>
                    </svg>
                    {"Processing ..."}
                </button>

                <button type="submit" hidden={login.loading()}
                    class="w-full rounded-md mt-7 mb-3 py-2.5 text-sm font-semibold text-white bg-sky-600 hover:bg-sky-700 active:bg-sky-600">
                    {"Log In"}
                </button>

                <div class="mt-1 flex items-center">
                    <input id="remember-me" type="checkbox" checked={*props.persist} {onclick} class="h-4 w-4 rounded" />
                    <label class="grow ml-2 text-sm text-slate-600" for="remember-me">{"Remember me"}</label>
                    <a class="text-sm text-sky-700" href="#">{"Forget password?"}</a>
                </div>
            </form>
            <div class="text-sm text-center">
                <span class="text-slate-600">{"Don't have an account? "}</span>
                <a class="text-sky-700" href="/register">{"Register here."}</a>
            </div>
        </div>
    }
}
