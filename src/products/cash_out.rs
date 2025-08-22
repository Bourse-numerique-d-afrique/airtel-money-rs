use crate::{
    authorization::get_valid_access_token,
    requests::cash_out_request_payment::{
        AdditionalInfo, CashOutRequestPayment, Subscriber, Transaction,
    },
    responses::cash_out_response::CashOutResponse,
    Country, Currency, Environment,
};

pub struct CashOut {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl CashOut {
    /*
       * Create a new instance of CashOut
       @param country: Country
       @param currency: Currency
       @param environment: Environment
       @param client_id: String
       @param client_secret: String
       @return CashOut
    */
    pub fn new(
        country: Country,
        currency: Currency,
        environment: Environment,
        client_id: String,
        client_secret: String,
    ) -> Self {
        CashOut {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /*
       * Cash out
       @param msisdn: String - subscriber's mobile number
       @param amount: i32 - amount to cash out
       @param transaction_id: String - unique transaction ID
       @param reference: String - transaction reference
       @param pin: String - PIN for the transaction
       @param remark: String - transaction remark
       @return Result<CashOutResponse, Box<dyn std::error::Error>>
    */
    pub async fn cash_out(
        &self,
        msisdn: String,
        amount: i32,
        transaction_id: String,
        reference: String,
        pin: String,
        remark: String,
    ) -> Result<CashOutResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .post(format!("{}/standard/v1/cashout/", self.environment))
            .bearer_auth(access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string())
            .header("x-signature", "signature_placeholder")
            .header("x-key", "api_key_placeholder")
            .body(CashOutRequestPayment {
                reference,
                subscriber: Subscriber { msisdn },
                transaction: Transaction {
                    amount,
                    id: transaction_id,
                },
                additional_info: vec![AdditionalInfo {
                    key: "remark".to_string(),
                    value: remark,
                }],
                pin,
            });
        let res = req.send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let response: CashOutResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }

    /*
       * Get the status of a cash out
       @param id: String - transaction ID to check status for
       @return Result<CashOutResponse, Box<dyn std::error::Error>>
    */
    pub async fn get_status(
        &self,
        id: String,
    ) -> Result<CashOutResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .get(format!(
                "{}/standard/v1/payments/{}",
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
            let response: CashOutResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }
}
