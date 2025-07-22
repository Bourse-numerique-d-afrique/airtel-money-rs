
// {
//     "subscriber": {
//       "msisdn": "70***14"
//     },
//     "transaction": {
//       "amount": "100",
//       "id": "12***260"
//     },
//     "additional_info": [
//       {
//         "key": "remark",
//         "value": "AIRTXXXXXX"
//       }
//     ],
//     "reference": "10***2",
//     "pin": "KYJE***+o8="
//   }

use serde::{Serialize, Deserialize};

#[doc(hidden)]
use reqwest::Body;

#[derive(Debug, Serialize, Deserialize)]
pub struct CashInPaymentRequest {
    pub subscriber: Subscriber,
    pub transaction: Transaction,
    pub additional_info: Vec<AdditionalInfo>,
    pub reference: String,
    pub pin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub msisdn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub amount: i32,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalInfo {
    pub key: String,
    pub value: String,
}

impl From<CashInPaymentRequest> for Body {
    fn from(disbursement_payment_request: CashInPaymentRequest) -> Self {
        let t = format!(
            r#"{{"subscriber":{{"msisdn":"{}"}},"transaction":{{"amount":{},"id":"{}"}},"additional_info":{},"reference":"{}","pin":"{}"}}"#,
            disbursement_payment_request.subscriber.msisdn,
            disbursement_payment_request.transaction.amount,
            disbursement_payment_request.transaction.id,
            serde_json::to_string(&disbursement_payment_request.additional_info).unwrap(),
            disbursement_payment_request.reference,
            disbursement_payment_request.pin
        );
        Body::from(t)
    }
}
