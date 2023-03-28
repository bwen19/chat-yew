use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{
    classes, function_component, html, use_node_ref, AttrValue, Callback, Html, Properties,
    UseStateHandle, use_state,
};

// ========================// SearchBox //======================== //

#[derive(PartialEq, Properties)]
pub struct SearchBoxProps {
    pub searching: UseStateHandle<bool>,
    pub onenter: Callback<String>,
    pub placeholder: AttrValue,
}

#[function_component]
pub fn SearchBox(props: &SearchBoxProps) -> Html {
    let input_value = use_state(String::default);
    let input_node_ref = use_node_ref();

    let oninput = {
        let searching = props.searching.clone();
        let input_value = input_value.clone();
        let input_node_ref = input_node_ref.clone();

        move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                input_value.set(input.value());
                if input.value().is_empty() {
                    searching.set(false);
                }
            }
        }
    };

    let onkeyup = {
        let onenter = props.onenter.clone();
        let searching = props.searching.clone();
        let input_value = input_value.clone();

        move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                searching.set(true);
                onenter.emit((*input_value).clone());
            }
        }
    };

    let onclear = {
        let input_value = input_value.clone();
        let searching = props.searching.clone();

        move |_| {
            input_value.set(String::default());
            searching.set(false);
        }
    };

    let cls = "absolute inset-y-0 right-0 cursor-pointer w-8 flex items-center justify-center";

    html! {
        <div class="relative">
            <div class="absolute inset-y-0 left-0 pointer-events-none w-8 flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                    stroke="currentColor" class="w-4 h-4 text-slate-500">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
                </svg>
            </div>
            <input type="text" spellcheck="false" ref={input_node_ref} value={(*input_value).clone()} {oninput} {onkeyup}
                class="w-full rounded-lg text-sm px-8 py-2 border-0 placeholder:text-slate-400 text-slate-800 leading-tight bg-slate-300 focus:bg-white focus:ring-2 focus:ring-sky-600 focus:outline-none"
                placeholder={props.placeholder.clone()} />
            <div onclick={onclear} class={classes!(cls, (*input_value).is_empty().then(|| "invisible"))}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2"
                    stroke="currentColor" class="w-4 h-4 text-slate-700">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </div>
        </div>
    }
}
