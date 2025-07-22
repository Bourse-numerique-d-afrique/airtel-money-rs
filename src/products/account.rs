use crate::{
    authorization::get_valid_access_token,
    responses::account_balance_response::AccountBalanceResponse, Country, Currency, Environment,
};

pub struct Account {
    pub country: Country,
    pub currency: Currency,
    pub environment: Environment,
    pub client_id: String,
    pub client_secret: String,
}

impl Account {
    /*
       * Create a new instance of Account
       @param country: Country
       @param currency: Currency
       @param environment: Environment
       @param client_id: String
       @param client_secret: String
       @return Account

    */
    pub fn new(
        country: Country,
        currency: Currency,
        environment: Environment,
        client_id: String,
        client_secret: String,
    ) -> Account {
        Account {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /*
       * Get the balance of the account
       @return Result<AccountBalanceResponse, Box<dyn std::error::Error>>

    */
    pub async fn get_balance(&self) -> Result<AccountBalanceResponse, Box<dyn std::error::Error>> {
        let token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/standard/v1/users/balance", self.environment))
            .bearer_auth(token.access_token)
            .header("Accept", "*/*")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string())
            .send()
            .await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let balance: AccountBalanceResponse = serde_json::from_str(&body)?;
            return Ok(balance);
        }
        let body = res.text().await?;
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            body,
        )));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    #[ignore = "requires API credentials"]
    async fn test_get_balance() {
        dotenv().ok();
        let client_id = env::var("AIRTEL_CLIENT_ID").expect("MTN_COLLECTION_URL must be set");
        let client_secret =
            env::var("AIRTEL_CLIENT_SECRET").expect("MTN_COLLECTION_URL must be set");
        let account = Account::new(
            Country::Kenya,
            Currency::KES,
            Environment::Sandbox,
            client_id,
            client_secret,
        );
        let balance = account.get_balance().await.unwrap();
        assert_eq!(balance.data.balance, "1000");
    }
}
