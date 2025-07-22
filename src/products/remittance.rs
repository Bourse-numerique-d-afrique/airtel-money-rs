use crate::{ 
    authorization::get_valid_access_token, 
    requests::{
        remittance_eligibility_request::RemittanceEligibilityRequest,
        remittance_transfer_credit_request::RemittanceTransferCreditRequest,
        remittance_transfer_status_request::RemittanceTransferStatusRequest,
        remittance_refund_request::RemittanceRefundRequest,
    },
    responses::{
        remittance_eligibility_response::RemittanceEligibilityResponse,
        remittance_transfer_response::RemittanceTransferResponse,
        remittance_status_response::RemittanceStatusResponse,
        remittance_refund_response::RemittanceRefundResponse,
    },
    Country, Currency, Environment
};


pub struct Remittance {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl Remittance {
    /*
        * Create a new instance of Remittance
        @param country: Country
        @param currency: Currency
        @param environment: Environment
        @param client_id: String
        @param client_secret: String
        @return Remittance
     */
    pub fn new(country: Country, currency: Currency, environment: Environment, client_id: String, client_secret: String) -> Self {
        Remittance {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }


    /*
        * Check eligibility for remittance
        @param msisdn: String - recipient's mobile number
        @param amount: i32 - amount to transfer
        @param country: Country - recipient's country
        @param currency: Currency - transfer currency
        @return Result<RemittanceEligibilityResponse, Box<dyn std::error::Error>>
     */
    pub async fn check_eligibility(&self, msisdn: String, amount: i32, country: Country, currency: Currency) -> Result<RemittanceEligibilityResponse, Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.get(format!("{}/openapi/moneytransfer/v2/validate", self.environment))
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .body(
            RemittanceEligibilityRequest {
                amount,
                country,
                currency,
                msisdn,
            }
        )
        .send().await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let response: RemittanceEligibilityResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Get money transfer status
        @param ext_tr_id: String - external transaction reference ID
        @return Result<RemittanceStatusResponse, Box<dyn std::error::Error>>
     */
    pub async fn money_transfer_status(&self, ext_tr_id: String) -> Result<RemittanceStatusResponse, Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.post(format!("{}/openapi/moneytransfer/v2/checkstatus/", self.environment))
        .bearer_auth(token.access_token)
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .body(
            RemittanceTransferStatusRequest::new(self.country, ext_tr_id)
        )
        .send().await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let response: RemittanceStatusResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Credit a money transfer
        @param amount: i32 - amount to transfer
        @param ext_trid: String - external transaction reference ID
        @param msisdn: String - recipient's mobile number
        @param payer_country: String - payer's country
        @param payer_first_name: String - payer's first name
        @param payer_last_name: String - payer's last name
        @param pin: String - PIN for authentication
        @return Result<RemittanceTransferResponse, Box<dyn std::error::Error>>
     */
    pub async fn money_transfer_credit(&self, amount: i32, ext_trid: String, msisdn: String, payer_country: String, payer_first_name: String, payer_last_name: String, pin: String) -> Result<RemittanceTransferResponse, Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.post(format!("{}/openapi/moneytransfer/v2/credit", self.environment))
        .bearer_auth(token.access_token)
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .body(
            RemittanceTransferCreditRequest {
                amount,
                country: self.country,
                currency: self.currency,
                ext_trid,
                msisdn,
                payer_country,
                payer_first_name,
                payer_last_name,
                pin,
            }
        )
        .send().await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let response: RemittanceTransferResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }

    /*
        * Refund a money transfer
        @param txn_id: String - transaction ID to refund
        @param pin: String - PIN for authentication
        @return Result<RemittanceRefundResponse, Box<dyn std::error::Error>>
     */
    pub async fn refund(&self, txn_id: String, pin: String) -> Result<RemittanceRefundResponse, Box<dyn std::error::Error>> {
        let token = get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client.post(format!("{}/openapi/moneytransfer/v2/refund", self.environment))
        .bearer_auth(token.access_token)
        .header("Content-type", "application/json")
        .header("Accept", "*/*")
        .body(
            RemittanceRefundRequest {
                country: self.country,
                txn_id,
                pin,
            }
        )
        .send().await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let response: RemittanceRefundResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, res.text().await?)))
        }
    }
}