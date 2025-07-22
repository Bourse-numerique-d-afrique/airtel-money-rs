// {
//     "subscriber": {
//       "msisdn": "70***14"
//     },
//     "transaction": {
//         "amount": "100",
//         "id": "12***260"
//     },
//       "additional_info": [
//           {
//               "key": "remark",
//               "value": "AIRTXXXXXX"
//           }
//         ],
//       "reference": "10***2"
//   }

use serde::{Serialize, Deserialize};
use reqwest::Body;


#[derive(Debug, Serialize, Deserialize)]
pub struct CashOutRequestPayment {
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

impl From<CashOutRequestPayment> for Body {
    fn from(cash_out_request_payment: CashOutRequestPayment) -> Self {
        let t = format!(
            r#"{{"subscriber":{{"msisdn":"{}"}},"transaction":{{"amount":{},"id":"{}"}},"additional_info":{},"reference":"{}","pin":"{}"}}"#,
            cash_out_request_payment.subscriber.msisdn,
            cash_out_request_payment.transaction.amount,
            cash_out_request_payment.transaction.id,
            serde_json::to_string(&cash_out_request_payment.additional_info).unwrap(),
            cash_out_request_payment.reference,
            cash_out_request_payment.pin
        );
        Body::from(t)
    }
}