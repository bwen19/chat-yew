use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;
use yew::{hook, use_effect_with_deps, Callback, NodeRef};

// ========================// use_click_away //======================== //

#[hook]
pub fn use_click_away(node_ref: NodeRef, cb: Callback<()>) {
    let node = node_ref.clone();

    use_effect_with_deps(
        move |_| {
            let document = web_sys::window().unwrap_throw().document().unwrap_throw();
            let listener = EventListener::new(&document, "mousedown", move |e| {
                if let Some(element) = e.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                    if let Some(node) = node.get() {
                        if !node.contains(Some(&*element)) {
                            cb.emit(());
                        }
                    }
                }
            });

            move || drop(listener)
        },
        node_ref,
    );
}
