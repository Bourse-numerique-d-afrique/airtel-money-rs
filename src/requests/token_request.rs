#[doc(hidden)]
use reqwest::Body;

#[doc(hidden)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

impl From<TokenRequest> for Body {
    fn from(access_token_request: TokenRequest) -> Self {
        let t = format!(
            "client_id={}&client_secret={}&grant_type={}",
            access_token_request.client_id,
            access_token_request.client_secret,
            access_token_request.grant_type
        );
        Body::from(t)
    }
}
