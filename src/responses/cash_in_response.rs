use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CashInResponse {
    pub data: Data,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub transaction: Transaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub reference_id: String,
    pub airtel_money_id: String,
    pub id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub code: String,
    pub message: String,
    pub result_code: String,
    pub response_code: String,
    pub success: bool,
}
