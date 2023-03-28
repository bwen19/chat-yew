use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct LoginModalProps {}

#[function_component]
pub fn LoginModal(props: &LoginModalProps) -> Html {
    let LoginModalProps {} = props;

    html! {
        <div class="relative z-20" role="auth">
            <div class="fixed inset-0 bg-slate-400 bg-opacity-75 transition-opacity"></div>
            <div class="fixed inset-0 p-5 flex min-h-full items-center justify-center">
                <div class="w-96 transform overflow-hidden rounded-lg shadow-xl transition-all">
                    <div class="bg-white px-4 py-5 space-y-4">
                        <div class="flex items-center justify-center space-x-4">
                            <div class="rounded-full w-10 h-10 bg-red-100 flex items-center justify-center">
                                <svg class="h-6 w-6 text-rose-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                    stroke="currentColor" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round"
                                        d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                                </svg>
                            </div>
                            <h3 class="text-base font-semibold leading-6 text-slate-900">
                                {"Login expired"}</h3>
                        </div>
                        <div class="p-4 flex items-center justify-center space-x-3">
                            <label for="input-passwd" class="text-sm font-semibold text-slate-800">{"Password"}</label>
                            <input id="input-passwd" type="password"
                                class="w-full px-3 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-sky-600" />
                        </div>
                    </div>
                    <div class="bg-slate-100 px-5 py-3 flex items-center justify-end space-x-4">
                        <button type="button"
                            class="rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-slate-300 hover:bg-slate-200">{"Cancel"}</button>
                        <button type="button"
                            class="rounded-md bg-sky-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-sky-500">{"Submit"}</button>
                    </div>
                </div>
            </div>

        </div>
    }
}
