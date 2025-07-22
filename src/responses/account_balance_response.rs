use crate::Currency;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    pub data: Data,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub balance: String,
    pub currency: Currency,
    pub account_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub code: String,
    pub message: String,
    pub result_code: String,
    pub response_code: String,
    pub success: bool,
}
