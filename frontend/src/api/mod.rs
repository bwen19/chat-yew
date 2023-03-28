pub mod auth;
pub mod config;
pub mod private;

// ========================// ApiError //======================== //

/// Process of responding error from server
#[derive(PartialEq, Eq)]
pub enum ApiError {
    // error message displayed to users
    Toast(String),
    // print to the console
    Console,
    // error of expired token
    Expire,
}

fn to_console<E>(err: E) -> ApiError
where
    E: std::error::Error,
{
    gloo_console::error!(err.to_string());
    ApiError::Console
}
