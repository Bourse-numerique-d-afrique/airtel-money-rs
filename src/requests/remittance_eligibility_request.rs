#[doc(hidden)]
use reqwest::Body;

use crate::{Country, Currency};

#[derive(Debug)]
pub struct RemittanceEligibilityRequest {
    pub amount: i32,
    pub country: Country,
    pub currency: Currency,
    pub msisdn: String,
}

impl From<RemittanceEligibilityRequest> for Body {
    fn from(remittance_eligibility_request: RemittanceEligibilityRequest) -> Self {
        let t = format!(
            "amount={}&country={}&currency={}&msisdn={}",
            remittance_eligibility_request.amount,
            remittance_eligibility_request.country,
            remittance_eligibility_request.currency,
            remittance_eligibility_request.msisdn
        );
        Body::from(t)
    }
}
