use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemittanceEligibilityResponse {
    pub data: Data,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub eligible: bool,
    pub msisdn: String,
    pub country: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub code: String,
    pub message: String,
    pub result_code: String,
    pub response_code: String,
    pub success: bool,
}
