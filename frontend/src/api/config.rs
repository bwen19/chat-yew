use gloo_net::http::Method;

// ========================// ApiConfig //======================== //

macro_rules! api_config {
    (
        $(
            ($konst:ident, $url:expr, $method:ident);
        )+
    ) => {
        #[derive(Debug)]
        pub enum ApiConfig {
            $(
                $konst,
            )+
        }

        impl ApiConfig {
            #[inline]
            pub fn params(&self) -> (&'static str, Method) {
                match self {
                    $(
                        ApiConfig::$konst => ($url, Method::$method),
                    )+
                }
            }
        }
    };
}

api_config! {
    (Register, "/api/auth/register", POST);
    (Login, "/api/auth/login", POST);
    (AutoLogin, "/api/auth/auto-login", POST);
    (RenewToken, "/api/auth/renew-token", POST);
    (Logout, "/api/auth/logout", POST);
    (GetUserByName, "/api/user/username", GET);
}
