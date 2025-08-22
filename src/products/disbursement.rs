use crate::{
    authorization::get_valid_access_token,
    requests::disbursement_payment_request::{DisbursementPaymentRequest, Payee, Transaction},
    responses::disbursement_payment_response::DisbursementPaymentResponse,
    Country, Currency, Environment,
};

pub struct Disbursement {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl Disbursement {
    /*
       * Create a new instance of Disbursement
       @param country: Country
       @param currency: Currency
       @param environment: Environment
       @param client_id: String
       @param client_secret: String
       @return Disbursement
    */
    pub fn new(
        country: Country,
        currency: Currency,
        environment: Environment,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Disbursement {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /*
       * Disburse
       @param msisdn: String - recipient's mobile number
       @param amount: i32 - amount to disburse
       @param transaction_id: String - unique transaction ID
       @param reference: String - transaction reference
       @param pin: String - PIN for the transaction
       @return Result<DisbursementPaymentResponse, Box<dyn std::error::Error>>
    */
    pub async fn disburse(
        &self,
        msisdn: String,
        amount: i32,
        transaction_id: String,
        reference: String,
        pin: String,
    ) -> Result<DisbursementPaymentResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .post(format!("{}/standard/v1/disbursements/", self.environment))
            .bearer_auth(access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string())
            .header("x-signature", "signature_placeholder")
            .header("x-key", "api_key_placeholder")
            .body(DisbursementPaymentRequest {
                payee: Payee { msisdn },
                reference,
                pin,
                transaction: Transaction {
                    amount,
                    id: transaction_id,
                },
            });
        let res = req.send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let response: DisbursementPaymentResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }

    /*
       * Get Status
       @param id: String - transaction ID to check status for
       @return Result<DisbursementPaymentResponse, Box<dyn std::error::Error>>
    */
    pub async fn get_status(
        &self,
        id: String,
    ) -> Result<DisbursementPaymentResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .get(format!("{}/standard/v1/disburse/{}", self.environment, id))
            .bearer_auth(access_token.access_token)
            .header("Accept", "*/*")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string());

        let res = req.send().await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let response: DisbursementPaymentResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }
}
