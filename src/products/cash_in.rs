use crate::{
    authorization::get_valid_access_token,
    requests::cash_in_payment_request::{
        AdditionalInfo, CashInPaymentRequest, Subscriber, Transaction,
    },
    responses::cash_in_response::CashInResponse,
    Country, Currency, Environment,
};

pub struct CashIn {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl CashIn {
    /*
       * Create a new instance of CashIn
       @param country: Country
       @param currency: Currency
       @param environment: Environment
       @param client_id: String
       @param client_secret: String
       @return CashIn
    */
    pub fn new(
        country: Country,
        currency: Currency,
        environment: Environment,
        client_id: String,
        client_secret: String,
    ) -> Self {
        CashIn {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /*
       * Cash in
       @param msisdn: String - subscriber's mobile number
       @param amount: i32 - amount to cash in
       @param transaction_id: String - unique transaction ID
       @param reference: String - transaction reference
       @param pin: String - PIN for the transaction
       @param remark: String - transaction remark
       @return Result<CashInResponse, Box<dyn std::error::Error>>
    */
    pub async fn cash_in(
        &self,
        msisdn: String,
        amount: i32,
        transaction_id: String,
        reference: String,
        pin: String,
        remark: String,
    ) -> Result<CashInResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .post(format!("{}/standard/v2/cashin/", self.environment))
            .bearer_auth(access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string())
            .header("x-signature", "signature_placeholder")
            .header("x-key", "api_key_placeholder")
            .body(CashInPaymentRequest {
                subscriber: Subscriber { msisdn },
                transaction: Transaction {
                    amount,
                    id: transaction_id,
                },
                additional_info: vec![AdditionalInfo {
                    key: "remark".to_string(),
                    value: remark,
                }],
                reference,
                pin,
            });
        let res = req.send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let response: CashInResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }

    /*
       * Get the status of a cash in
       @param id: String - transaction ID to check status for
       @return Result<CashInResponse, Box<dyn std::error::Error>>
    */
    pub async fn get_status(
        &self,
        id: String,
    ) -> Result<CashInResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .get(format!(
                "{}/standard/v1/cashin/{}",
                self.environment,
                id
            ))
            .bearer_auth(access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string());
        let res = req.send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let response: CashInResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }
}
