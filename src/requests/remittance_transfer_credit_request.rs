use reqwest::Body;

use crate::{Country, Currency};

pub struct RemittanceTransferCreditRequest {
    pub amount: i32,
    pub country: Country,
    pub currency: Currency,
    pub ext_trid: String,
    pub msisdn: String,
    pub payer_country: String,
    pub payer_first_name: String,
    pub payer_last_name: String,
    pub pin: String,
}

impl From<RemittanceTransferCreditRequest> for Body {
    fn from(remittance_transfer_credit_request: RemittanceTransferCreditRequest) -> Self {
        let t = format!("amount={}&country={}&currency={}&extTRID={}&msisdn={}&payerCountry={}&payerFirstName={}&payerLastName={}&pin={}", remittance_transfer_credit_request.amount, remittance_transfer_credit_request.country, remittance_transfer_credit_request.currency, remittance_transfer_credit_request.ext_trid, remittance_transfer_credit_request.msisdn, remittance_transfer_credit_request.payer_country, remittance_transfer_credit_request.payer_first_name, remittance_transfer_credit_request.payer_last_name, remittance_transfer_credit_request.pin);
        Body::from(t)
    }
}
