use airtel_rs::{AirtelMoney, Environment, Country, Currency};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airtel_money_creation() {
        let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
        assert_eq!(airtel.get_country(), Country::Kenya);
        assert_eq!(airtel.get_currency(), Currency::KES);
        assert_eq!(airtel.environment, Environment::Sandbox);
    }

    #[test]
    fn test_airtel_money_all_countries() {
        let test_cases = vec![
            (Country::Uganda, Currency::UGX),
            (Country::Kenya, Currency::KES),
            (Country::Tanzania, Currency::TZS),
            (Country::Madagascar, Currency::MGA),
            (Country::DRC, Currency::CDF),
            (Country::Zambia, Currency::ZMW),
            (Country::Seychelles, Currency::SCR),
            (Country::Rwanda, Currency::RWF),
            (Country::Malawi, Currency::MWK),
            (Country::Nigeria, Currency::NGN),
            (Country::Niger, Currency::XOF),
            (Country::Chad, Currency::XAF),
            (Country::Gabon, Currency::XAF),
            (Country::CongoB, Currency::XAF),
        ];

        for (country, expected_currency) in test_cases {
            let airtel = AirtelMoney::new(Environment::Sandbox, country);
            assert_eq!(airtel.get_country(), country);
            assert_eq!(airtel.get_currency(), expected_currency);
        }
    }

    #[test]
    fn test_product_accessors() {
        let airtel = AirtelMoney::new(Environment::Production, Country::Uganda);
        let client_id = "test_client_id".to_string();
        let client_secret = "test_client_secret".to_string();

        // Test that all product accessors work
        let collection = airtel.collection(client_id.clone(), client_secret.clone());
        assert_eq!(collection.country, Country::Uganda);
        assert_eq!(collection.currency, Currency::UGX);
        assert_eq!(collection.environment, Environment::Production);

        let disbursement = airtel.disbursement(client_id.clone(), client_secret.clone());
        assert_eq!(disbursement.country, Country::Uganda);
        assert_eq!(disbursement.currency, Currency::UGX);

        let remittance = airtel.remittance(client_id.clone(), client_secret.clone());
        assert_eq!(remittance.country, Country::Uganda);
        assert_eq!(remittance.currency, Currency::UGX);

        let cash_in = airtel.cash_in(client_id.clone(), client_secret.clone());
        assert_eq!(cash_in.country, Country::Uganda);
        assert_eq!(cash_in.currency, Currency::UGX);

        let cash_out = airtel.cash_out(client_id.clone(), client_secret.clone());
        assert_eq!(cash_out.country, Country::Uganda);
        assert_eq!(cash_out.currency, Currency::UGX);

        let account = airtel.account(client_id, client_secret);
        assert_eq!(account.country, Country::Uganda);
        assert_eq!(account.currency, Currency::UGX);
    }

    #[test]
    fn test_environments() {
        let sandbox = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
        let production = AirtelMoney::new(Environment::Production, Country::Kenya);
        
        assert_eq!(sandbox.environment, Environment::Sandbox);
        assert_eq!(production.environment, Environment::Production);
    }

    // Unit tests for request/response structures would go here
    // These tests verify the structure compilation without making actual API calls
    
    #[test]
    fn test_country_display() {
        assert_eq!(format!("{}", Country::Kenya), "KE");
        assert_eq!(format!("{}", Country::Uganda), "UG");
        assert_eq!(format!("{}", Country::Tanzania), "TZ");
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(format!("{}", Currency::KES), "KES");
        assert_eq!(format!("{}", Currency::UGX), "UGX");
        assert_eq!(format!("{}", Currency::TZS), "TZS");
    }
}

// Integration tests that require actual API credentials
// These are disabled by default but can be enabled with environment variables
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::env;

    fn get_test_credentials() -> Option<(String, String)> {
        let client_id = env::var("AIRTEL_CLIENT_ID").ok()?;
        let client_secret = env::var("AIRTEL_CLIENT_SECRET").ok()?;
        Some((client_id, client_secret))
    }

    #[tokio::test]
    #[ignore = "requires API credentials"]
    async fn test_token_generation() {
        if let Some((client_id, client_secret)) = get_test_credentials() {
            let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
            let _account = airtel.account(client_id, client_secret);
            
            // This would test actual token generation
            // let balance = account.get_balance().await;
            // assert!(balance.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires API credentials"]
    async fn test_collection_ussd_push() {
        if let Some((client_id, client_secret)) = get_test_credentials() {
            let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
            let _collection = airtel.collection(client_id, client_secret);
            
            // Test USSD push with sandbox credentials
            // let result = collection.ussd_push(
            //     "test_ref_123".to_string(),
            //     "254700000000".to_string(),
            //     100,
            //     "test_tx_123".to_string()
            // ).await;
            // assert!(result.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires API credentials"]
    async fn test_disbursement() {
        if let Some((client_id, client_secret)) = get_test_credentials() {
            let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
            let _disbursement = airtel.disbursement(client_id, client_secret);
            
            // Test disbursement with sandbox credentials
            // let result = disbursement.disburse(
            //     "254700000000".to_string(),
            //     100,
            //     "test_tx_123".to_string(),
            //     "test_ref_123".to_string(),
            //     "1234".to_string()
            // ).await;
            // assert!(result.is_ok());
        }
    }
}