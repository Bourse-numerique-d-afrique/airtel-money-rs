use reqwest::Body;

use crate::Country;

pub struct RemittanceRefundRequest {
    pub country: Country,
    pub txn_id: String,
    pub pin: String,
}

impl From<RemittanceRefundRequest> for Body {
    fn from(remittance_refund_request: RemittanceRefundRequest) -> Self {
        let t = format!(
            "country={}&txnID={}&pin={}",
            remittance_refund_request.country,
            remittance_refund_request.txn_id,
            remittance_refund_request.pin
        );
        Body::from(t)
    }
}
