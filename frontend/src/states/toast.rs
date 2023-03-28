use std::rc::Rc;
use yew::{AttrValue, Reducible, UseReducerHandle};

// ========================// ToastState //======================== //

#[derive(Default, PartialEq)]
pub struct ToastState {
    pub open: bool,
    pub color: AttrValue,
    pub message: AttrValue,
}

pub enum ToastAction {
    Info(AttrValue),
    Error(AttrValue),
    Close,
}

impl Reducible for ToastState {
    type Action = ToastAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ToastAction::Info(message) => Self {
                open: true,
                message,
                color: AttrValue::from("bg-sky-600"),
            }
            .into(),
            ToastAction::Error(message) => Self {
                open: true,
                message,
                color: AttrValue::from("bg-orange-600"),
            }
            .into(),
            ToastAction::Close => Self {
                open: false,
                message: AttrValue::default(),
                color: AttrValue::default(),
            }
            .into(),
        }
    }
}

pub type ToastContext = UseReducerHandle<ToastState>;
