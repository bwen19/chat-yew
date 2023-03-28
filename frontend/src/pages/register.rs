use crate::hooks::use_register;
use shared::auth::RegisterRequest;
use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Html, NodeRef};

// ========================// Register //======================== //

#[derive(Default)]
struct InputForm {
    username: NodeRef,
    password: NodeRef,
    code: NodeRef,
}

#[function_component]
pub fn Register() -> Html {
    let register = use_register();
    let input = use_state(InputForm::default);

    let submit = {
        let input = input.clone();
        let register = register.clone();
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
            let code = input
                .code
                .cast::<HtmlInputElement>()
                .map(|x| x.value())
                .unwrap_or("".to_owned());

            if username.is_empty() || password.is_empty() || code.is_empty() {
                return;
            }
            let arg = RegisterRequest {
                username,
                password,
                code,
            };
            register.run(arg);
        }
    };

    html! {
        <div class="h-screen w-full px-2 bg-no-repeat bg-center bg-cover flex justify-center items-center"
            style="background-image: url('/assets/pic/register-bg.jpg')">
            <div class="max-w-md lg:max-w-4xl w-full flex rounded-lg overflow-hidden">
                <div class="basis-1/2 shrink-0 hidden lg:block">
                    <img src="/assets/pic/register-left.png" alt="register" class="h-full object-cover object-center" />
                </div>
                <div class="grow relative h-full bg-slate-50 flex items-center justify-center">
                    <div class="w-full space-y-8 p-16">
                        <h2 class="text-slate-700 text-2xl font-bold text-center">{"Register"}</h2>
                        <form onsubmit={submit} action="javascript:void(0);">
                            <label class="block mb-2 text-sm font-semibold text-slate-500"
                                for="inline-username">{"Username"}</label>
                            <input id="inline-username" type="text" minlength="2" ref={&input.username}
                                class="appearance-none w-full py-2 px-4 rounded-md bg-white text-slate-700 leading-tight border-2 border-slate-300 focus:outline-none invalid:border-rose-500 focus:border-sky-600 focus:invalid:border-rose-500" />

                            <label class="block mt-3 mb-2 text-sm font-semibold text-slate-500"
                                for="inline-password">{"Password"}</label>
                            <input id="inline-password" type="password" minlength="6" ref={&input.password}
                                class="appearance-none w-full py-2 px-4 rounded-md bg-white text-slate-700 leading-tight border-2 border-slate-300 focus:outline-none invalid:border-rose-500 focus:border-sky-600 focus:invalid:border-rose-500" />

                            <label class="block mt-3 mb-2 text-sm font-semibold text-slate-500"
                                for="inline-code">{"Invitation code"}</label>
                            <input id="inline-code" type="text" minlength="1" ref={&input.code}
                                class="appearance-none w-full py-2 px-4 rounded-md bg-white text-slate-700 leading-tight border-2 border-slate-300 focus:outline-none invalid:border-rose-500 focus:border-sky-600 focus:invalid:border-rose-500" />

                            <button type="button" disabled={true} hidden={!register.loading()}
                                class="w-full rounded-md mt-7 mb-3 py-2.5 px-4 cursor-wait text-sm font-semibold text-slate-400 bg-cyan-700">
                                <svg aria-hidden="true" role="status" class="inline w-4 h-4 mr-3 text-teal-700 animate-spin"
                                    viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                        fill="#E5E7EB" />
                                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                        fill="currentColor" />
                                </svg>
                                {"Submitting ..."}
                            </button>
                            <button type="submit" hidden={register.loading()}
                                class="w-full rounded-md mt-7 mb-3 py-2.5 text-sm font-semibold text-white bg-sky-600 hover:bg-sky-700 active:bg-sky-600">
                                {"Register"}
                            </button>
                        </form>
                        <div class="text-sm text-center">
                            <span class="text-slate-600">{"Already have an account? "}</span>
                            <a class="text-sky-700" href="/login">{"Login here."}</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
