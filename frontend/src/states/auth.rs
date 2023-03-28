use shared::{
    auth::{LoginResponse, RenewTokenResponse},
    user::UserInfo,
};
use std::rc::Rc;
use yew::prelude::*;

// ========================// AuthState //======================== //

#[derive(Default, PartialEq)]
pub struct AuthState {
    pub authorized: bool,
    pub user: UserInfo,
    pub token: String,
}

pub enum AuthAction {
    Set(LoginResponse),
    Renew(RenewTokenResponse),
    Clear,
}

impl Reducible for AuthState {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::Set(data) => Self {
                authorized: true,
                user: data.user,
                token: data.access_token,
            }
            .into(),
            AuthAction::Renew(data) => {
                let user = self.user.clone();
                Self {
                    authorized: self.authorized,
                    user,
                    token: data.access_token,
                }
                .into()
            }
            AuthAction::Clear => Self::default().into(),
        }
    }
}

pub type AuthContext = UseReducerHandle<AuthState>;
