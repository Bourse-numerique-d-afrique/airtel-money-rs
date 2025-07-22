use serde::{Deserialize, Serialize};

#[doc(hidden)]
use reqwest::Body;

#[derive(Debug, Serialize, Deserialize)]
pub struct DisbursementPaymentRequest {
    pub payee: Payee,
    pub reference: String,
    pub pin: String,
    pub transaction: Transaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payee {
    pub msisdn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub amount: i32,
    pub id: String,
}

impl From<DisbursementPaymentRequest> for Body {
    fn from(disbursement_payment_request: DisbursementPaymentRequest) -> Self {
        let t = format!(
            r#"{{"payee":{{"msisdn":"{}"}},"transaction":{{"amount":{},"id":"{}"}},"reference":"{}","pin":"{}"}}"#,
            disbursement_payment_request.payee.msisdn,
            disbursement_payment_request.transaction.amount,
            disbursement_payment_request.transaction.id,
            disbursement_payment_request.reference,
            disbursement_payment_request.pin
        );
        Body::from(t)
    }
}
