use crate::Country;
use crate::Currency;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
use reqwest::Body;

#[derive(Debug, Serialize, Deserialize)]
pub struct USSDSubscriberRequest {
    pub country: Country,
    pub msisdn: String,
    pub currency: Currency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct USSDTransactionRequest {
    pub amount: i32,
    pub country: Country,
    pub currency: Currency,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UssdPushRequest {
    pub reference: String,
    pub subscriber: USSDSubscriberRequest,
    pub transaction: USSDTransactionRequest,
}

impl From<UssdPushRequest> for Body {
    fn from(ussd_push_request: UssdPushRequest) -> Self {
        let t = format!(
            "reference={}&subscriber={}&transaction={}",
            ussd_push_request.reference,
            serde_json::to_string(&ussd_push_request.subscriber).unwrap(),
            serde_json::to_string(&ussd_push_request.transaction).unwrap()
        );
        Body::from(t)
    }
}
