use serde::{Deserialize, Serialize};

#[doc(hidden)]
use reqwest::Body;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundCollectionRequest {
    pub airtel_money_id: String,
}

impl From<RefundCollectionRequest> for Body {
    fn from(refund_collection_request: RefundCollectionRequest) -> Self {
        let t = format!(
            "airtel_money_id={}",
            refund_collection_request.airtel_money_id
        );
        Body::from(t)
    }
}
