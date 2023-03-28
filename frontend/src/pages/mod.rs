mod chat;
mod login;
mod page_not_found;
mod register;
pub mod route;

#[derive(Clone, PartialEq)]
pub enum ChatPage {
    Room,
    Friend,
}
