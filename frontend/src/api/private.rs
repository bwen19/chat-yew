use super::{auth::AuthApi, config::ApiConfig, to_console, ApiError};
use crate::states::auth::{AuthAction, AuthState};
use gloo_net::http::{Method, Request};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use shared::auth::RenewTokenResponse;
use web_sys::RequestCredentials;
use yew::UseReducerHandle;

// ========================// PrivateApi //======================== //

#[derive(Clone)]
pub struct PrivateApi {
    url: String,
    method: Method,
    auth: UseReducerHandle<AuthState>,
}

impl PrivateApi {
    pub fn new(api: ApiConfig, auth: UseReducerHandle<AuthState>) -> Self {
        let (url, method) = api.params();
        Self {
            url: url.to_owned(),
            method,
            auth,
        }
    }

    /// Send http request to the server
    pub async fn send<T, D>(&self, payload: &T) -> Result<D, ApiError>
    where
        T: Serialize,
        D: DeserializeOwned,
    {
        let resp = self
            .build(payload, None)?
            .send()
            .await
            .map_err(to_console)?;

        if resp.ok() {
            resp.json::<D>().await.map_err(to_console)
        } else {
            let msg = resp.text().await.map_err(to_console)?;
            match resp.status() {
                400 => Err(ApiError::Toast(msg)),
                402 => {
                    let token = self.renew_token().await?;
                    self.resend(payload, token).await
                }
                _ => {
                    gloo_console::error!(msg);
                    Err(ApiError::Console)
                }
            }
        }
    }

    fn build<T>(&self, payload: &T, token: Option<String>) -> Result<Request, ApiError>
    where
        T: Serialize,
    {
        let request = if self.method.to_string() == "GET" {
            let query = match serde_json::to_value(&payload).map_err(to_console)? {
                Value::Object(req_map) => req_map
                    .into_iter()
                    .map(|(key, val)| format_map(key, val))
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<String>>()
                    .join("&"),
                _ => {
                    gloo_console::error!("failed to serialize payload");
                    return Err(ApiError::Console);
                }
            };

            let url = format!("{}?{}", self.url, query);
            Request::new(&url)
        } else {
            Request::new(&self.url).json(payload).map_err(to_console)?
        };

        let token = if let Some(token) = token {
            format!("Bearer {}", token)
        } else {
            format!("Bearer {}", self.auth.token)
        };

        let request = request
            .method(self.method)
            .header("Authorization", &token)
            .credentials(RequestCredentials::Omit);

        Ok(request)
    }

    async fn renew_token(&self) -> Result<String, ApiError> {
        let api = AuthApi::new(ApiConfig::RenewToken);
        match api.send::<(), RenewTokenResponse>(None).await {
            Ok(data) => {
                let token = data.access_token.clone();
                self.auth.dispatch(AuthAction::Renew(data));
                Ok(token)
            }
            Err(e) => {
                // TODO: if expired, add login dialog here
                self.auth.dispatch(AuthAction::Clear);
                Err(e)
            }
        }
    }

    async fn resend<T, D>(&self, payload: &T, token: String) -> Result<D, ApiError>
    where
        T: Serialize,
        D: DeserializeOwned,
    {
        let resp = self
            .build(payload, Some(token))?
            .send()
            .await
            .map_err(to_console)?;

        if resp.ok() {
            resp.json::<D>().await.map_err(to_console)
        } else {
            let msg = resp.text().await.map_err(to_console)?;
            match resp.status() {
                400 => Err(ApiError::Toast(msg)),
                402 => Err(ApiError::Expire),
                _ => {
                    gloo_console::error!(msg);
                    Err(ApiError::Console)
                }
            }
        }
    }
}

fn format_map(key: String, val: Value) -> String {
    let val = match val {
        Value::Bool(bl) => bl.to_string(),
        Value::Number(num) => num.to_string(),
        Value::String(s) => s,
        _ => {
            gloo_console::error!("invalid value");
            return "".to_owned();
        }
    };
    format!("{}={}", key, val)
}
