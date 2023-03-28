use yew::{classes, function_component, html, use_state_eq, AttrValue, Classes, Html, Properties};

// ========================// Avatar //======================== //

#[derive(PartialEq, Properties)]
pub struct AvatarProps {
    pub classes: Classes,
    pub image: AttrValue,
}

#[function_component]
pub fn Avatar(props: &AvatarProps) -> Html {
    let exist = use_state_eq(|| true);
    let onerror = {
        let exist = exist.clone();
        move |_| exist.set(false)
    };

    if *exist {
        html! {
            <img class={classes!("rounded-full", props.classes.clone())}
                src={props.image.clone()} alt="avatar" {onerror} />
        }
    } else {
        html! {
            <div class="bg-slate-400 rounded-full overflow-hidden text-slate-500">
                <svg class={classes!("rounded-full", props.classes.clone())}
                    fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"
                        clip-rule="evenodd"></path>
                </svg>
            </div>
        }
    }
}
