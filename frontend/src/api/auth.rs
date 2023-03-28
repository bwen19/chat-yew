use super::{config::ApiConfig, to_console, ApiError};
use gloo_net::http::{Method, Request};
use serde::{de::DeserializeOwned, Serialize};
use web_sys::RequestCredentials;

// ========================// AuthApi //======================== //

#[derive(Clone)]
pub struct AuthApi {
    url: String,
    method: Method,
}

impl AuthApi {
    pub fn new(api: ApiConfig) -> Self {
        let (url, method) = api.params();
        Self {
            url: url.to_owned(),
            method,
        }
    }

    fn build<T>(&self, payload: Option<&T>) -> Result<Request, ApiError>
    where
        T: Serialize,
    {
        let request = Request::new(&self.url)
            .method(self.method)
            .credentials(RequestCredentials::SameOrigin);

        let request = if let Some(req) = payload {
            request.json(req).map_err(to_console)?
        } else {
            request
        };

        Ok(request)
    }

    /// Send http request to the server
    pub async fn send<T, D>(&self, payload: Option<&T>) -> Result<D, ApiError>
    where
        T: Serialize,
        D: DeserializeOwned,
    {
        let resp = self.build(payload)?.send().await.map_err(to_console)?;

        if resp.ok() {
            resp.json::<D>().await.map_err(to_console)
        } else {
            let msg = resp.text().await.map_err(to_console)?;
            match resp.status() {
                400 => Err(ApiError::Toast(msg)),
                _ => {
                    gloo_console::error!(msg);
                    Err(ApiError::Console)
                }
            }
        }
    }
}
