use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemittanceTransferResponse {
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
    pub amount: String,
    pub currency: String,
    pub ext_trid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub code: String,
    pub message: String,
    pub result_code: String,
    pub response_code: String,
    pub success: bool,
}