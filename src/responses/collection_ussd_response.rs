use serde::{Deserialize, Serialize};

// {
//     "data": {
//         "transaction": {
//             "id": "A**********N5",
//             "status": "SUCCESS"
//         }
//     },
//     "status": {
//         "code": "200",
//         "message": "SUCCESS",
//         "result_code": "ESB000010",
//         "response_code": "DP00800001006",
//         "success": true
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionUSSDResponse {
    pub data: Data,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub transaction: Transaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
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
