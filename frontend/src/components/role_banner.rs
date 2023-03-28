use yew::{function_component, html, AttrValue, Html, Properties, classes};

// ========================// Friend //======================== //

#[derive(PartialEq, Properties)]
pub struct RoleBannerProps {
    pub role: AttrValue,
}

#[function_component]
pub fn RoleBanner(props: &RoleBannerProps) -> Html {
    let color = match props.role.as_str() {
        "admin" => "bg-orange-500",
        "user" => "bg-sky-500",
        _ => "bg-slate-500",
    };
    let cls = "rounded-lg px-2 py-1 capitalize text-slate-50 text-xs font-semibold".to_owned();

    html! {
        <span class={classes!(cls, color)}>
            {props.role.clone()}
        </span>
    }
}
