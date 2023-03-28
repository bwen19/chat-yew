use crate::states::toast::{ToastAction, ToastContext};
use gloo_timers::callback::Timeout;
use yew::{function_component, html, use_context, use_effect_with_deps, Html};

// ========================// Toast //======================== //

#[function_component]
pub fn Toast() -> Html {
    let toast = use_context::<ToastContext>().expect("no toast context");

    {
        let toast = toast.clone();
        use_effect_with_deps(
            move |_toast| {
                if _toast.open {
                    let toast = _toast.clone();
                    Timeout::new(5000, move || {
                        toast.dispatch(ToastAction::Close);
                    })
                    .forget();
                }
                move || ()
            },
            toast,
        );
    }

    let onclose = {
        let toast = toast.clone();
        move |_| toast.dispatch(ToastAction::Close)
    };

    html! {
        <div hidden={!toast.open} class="fixed inset-x-0 top-0 z-10">
            <div class={format!("mt-8 mx-auto max-w-md w-fit h-fit px-4 py-3 rounded-md shadow-lg flex space-x-4 {}", toast.color)}>
                <div class="text-slate-50">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
                    </svg>
                </div>
                <span class="text-sm text-white truncate min-w-0">
                    {toast.message.clone()}
                </span>
                <div onclick={onclose} class="text-slate-50 hover:text-slate-800 cursor-pointer">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </div>
            </div>
        </div>
    }
}
