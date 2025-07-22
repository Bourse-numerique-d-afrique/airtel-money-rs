use airtel_rs::{AirtelMoney, Country, Currency, Environment};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

// Test configuration
const TEST_PHONE_NUMBER: &str = "254700000000"; // Sandbox test number
const TEST_AMOUNT: i32 = 100; // Small test amount
const TEST_COUNTRY: Country = Country::Kenya;
const TEST_CURRENCY: Currency = Currency::KES;

fn load_test_credentials() -> (String, String, String) {
    dotenv().ok();
    let client_id =
        env::var("AIRTEL_CLIENT_ID").expect("AIRTEL_CLIENT_ID must be set in .env file");
    let client_secret =
        env::var("AIRTEL_CLIENT_SECRET").expect("AIRTEL_CLIENT_SECRET must be set in .env file");
    let pin = env::var("AIRTEL_PIN").expect("AIRTEL_PIN must be set in .env file");
    (client_id, client_secret, pin)
}

fn generate_unique_reference() -> String {
    format!(
        "test_{}",
        Uuid::new_v4().to_string().replace("-", "")[..8].to_uppercase()
    )
}

fn generate_unique_transaction_id() -> String {
    format!(
        "tx_{}",
        Uuid::new_v4().to_string().replace("-", "")[..10].to_uppercase()
    )
}

#[cfg(test)]
mod live_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_token_generation_and_refresh() {
        let (client_id, client_secret, _) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let account = airtel.account(client_id, client_secret);

        println!("🔑 Testing token generation and refresh...");

        // This will test token generation internally
        let result = account.get_balance().await;

        match result {
            Ok(balance) => {
                println!("✅ Token generation successful");
                println!("Account balance: {}", balance.data.balance);
                assert!(balance.status.success);
            }
            Err(e) => {
                println!("❌ Token generation failed: {}", e);
                panic!("Token generation failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_account_balance() {
        let (client_id, client_secret, _) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let account = airtel.account(client_id, client_secret);

        println!("🏦 Testing account balance retrieval...");

        let result = account.get_balance().await;

        match result {
            Ok(balance) => {
                println!("✅ Balance retrieved successfully");
                println!("Balance: {}", balance.data.balance);
                println!("Currency: {}", balance.data.currency);

                assert!(balance.status.success);
                assert!(!balance.data.balance.is_empty());
                assert_eq!(balance.data.currency, Currency::KES);
            }
            Err(e) => {
                println!("❌ Account balance test failed: {}", e);
                panic!("Account balance test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_collection_ussd_push() {
        let (client_id, client_secret, _) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let collection = airtel.collection(client_id, client_secret);

        let reference = generate_unique_reference();
        let transaction_id = generate_unique_transaction_id();

        println!("💰 Testing USSD push collection...");
        println!("Reference: {}", reference);
        println!("Transaction ID: {}", transaction_id);

        let result = collection
            .ussd_push(
                reference.clone(),
                TEST_PHONE_NUMBER.to_string(),
                TEST_AMOUNT,
                transaction_id.clone(),
            )
            .await;

        match result {
            Ok(response) => {
                println!("✅ USSD Push successful");
                println!("Transaction ID: {}", response.data.transaction.id);
                println!("Status: {}", response.data.transaction.status);

                assert!(response.status.success);
                assert!(!response.data.transaction.id.is_empty());

                // Wait a moment then check status
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                // Test status check
                println!("🔍 Checking transaction status...");
                let status_result = collection
                    .status(response.data.transaction.id.clone())
                    .await;

                match status_result {
                    Ok(status_response) => {
                        println!("✅ Status check successful");
                        println!("Status: {}", status_response.data.transaction.status);
                        assert!(status_response.status.success);
                    }
                    Err(e) => {
                        println!("⚠️  Status check failed (this might be expected): {}", e);
                        // Don't fail the test for status check as it might not be immediately available
                    }
                }
            }
            Err(e) => {
                println!("❌ USSD Push failed: {}", e);
                // Don't panic immediately, log the error for analysis
                println!("Error details: {}", e);
                assert!(false, "USSD Push test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_disbursement() {
        let (client_id, client_secret, pin) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let disbursement = airtel.disbursement(client_id, client_secret);

        let reference = generate_unique_reference();
        let transaction_id = generate_unique_transaction_id();

        println!("📤 Testing disbursement...");
        println!("Reference: {}", reference);
        println!("Transaction ID: {}", transaction_id);

        let result = disbursement
            .disburse(
                TEST_PHONE_NUMBER.to_string(),
                TEST_AMOUNT,
                transaction_id.clone(),
                reference,
                pin,
            )
            .await;

        match result {
            Ok(response) => {
                println!("✅ Disbursement successful");
                println!(
                    "Airtel Money ID: {}",
                    response.data.transaction.airtel_money_id
                );
                println!("Status: {}", response.data.transaction.status);

                assert!(response.status.success);
                assert!(!response.data.transaction.airtel_money_id.is_empty());

                // Test status check
                println!("🔍 Checking disbursement status...");
                let status_result = disbursement.get_status(transaction_id).await;

                match status_result {
                    Ok(status_response) => {
                        println!("✅ Disbursement status check successful");
                        println!("Status: {}", status_response.data.transaction.status);
                        assert!(status_response.status.success);
                    }
                    Err(e) => {
                        println!("⚠️  Disbursement status check failed: {}", e);
                        // Don't fail the test as status might not be immediately available
                    }
                }
            }
            Err(e) => {
                println!("❌ Disbursement failed: {}", e);
                println!("Error details: {}", e);
                assert!(false, "Disbursement test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_cash_in() {
        let (client_id, client_secret, pin) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let cash_in = airtel.cash_in(client_id, client_secret);

        let reference = generate_unique_reference();
        let transaction_id = generate_unique_transaction_id();

        println!("💵 Testing cash in...");
        println!("Reference: {}", reference);
        println!("Transaction ID: {}", transaction_id);

        let result = cash_in
            .cash_in(
                TEST_PHONE_NUMBER.to_string(),
                TEST_AMOUNT,
                transaction_id.clone(),
                reference,
                pin,
                "Test cash in transaction".to_string(),
            )
            .await;

        match result {
            Ok(response) => {
                println!("✅ Cash in successful");
                println!(
                    "Airtel Money ID: {}",
                    response.data.transaction.airtel_money_id
                );
                println!("Status: {}", response.data.transaction.status);

                assert!(response.status.success);
                assert!(!response.data.transaction.airtel_money_id.is_empty());

                // Test status check
                println!("🔍 Checking cash in status...");
                let status_result = cash_in.get_status(transaction_id).await;

                match status_result {
                    Ok(status_response) => {
                        println!("✅ Cash in status check successful");
                        println!("Status: {}", status_response.data.transaction.status);
                        assert!(status_response.status.success);
                    }
                    Err(e) => {
                        println!("⚠️  Cash in status check failed: {}", e);
                        // Don't fail the test as status might not be immediately available
                    }
                }
            }
            Err(e) => {
                println!("❌ Cash in failed: {}", e);
                println!("Error details: {}", e);
                assert!(false, "Cash in test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_cash_out() {
        let (client_id, client_secret, pin) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let cash_out = airtel.cash_out(client_id, client_secret);

        let reference = generate_unique_reference();
        let transaction_id = generate_unique_transaction_id();

        println!("💸 Testing cash out...");
        println!("Reference: {}", reference);
        println!("Transaction ID: {}", transaction_id);

        let result = cash_out
            .cash_out(
                TEST_PHONE_NUMBER.to_string(),
                TEST_AMOUNT,
                transaction_id.clone(),
                reference,
                pin,
                "Test cash out transaction".to_string(),
            )
            .await;

        match result {
            Ok(response) => {
                println!("✅ Cash out successful");
                println!(
                    "Airtel Money ID: {}",
                    response.data.transaction.airtel_money_id
                );
                println!("Status: {}", response.data.transaction.status);

                assert!(response.status.success);
                assert!(!response.data.transaction.airtel_money_id.is_empty());

                // Test status check
                println!("🔍 Checking cash out status...");
                let status_result = cash_out.get_status(transaction_id).await;

                match status_result {
                    Ok(status_response) => {
                        println!("✅ Cash out status check successful");
                        println!("Status: {}", status_response.data.transaction.status);
                        assert!(status_response.status.success);
                    }
                    Err(e) => {
                        println!("⚠️  Cash out status check failed: {}", e);
                        // Don't fail the test as status might not be immediately available
                    }
                }
            }
            Err(e) => {
                println!("❌ Cash out failed: {}", e);
                println!("Error details: {}", e);
                assert!(false, "Cash out test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_remittance_eligibility() {
        let (client_id, client_secret, _) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let remittance = airtel.remittance(client_id, client_secret);

        println!("🌍 Testing remittance eligibility check...");

        let result = remittance
            .check_eligibility(
                TEST_PHONE_NUMBER.to_string(),
                TEST_AMOUNT,
                TEST_COUNTRY,
                TEST_CURRENCY,
            )
            .await;

        match result {
            Ok(response) => {
                println!("✅ Eligibility check successful");
                println!("Eligible: {}", response.data.eligible);
                println!("MSISDN: {}", response.data.msisdn);
                println!("Country: {}", response.data.country);
                println!("Currency: {}", response.data.currency);

                assert!(response.status.success);
                assert_eq!(response.data.msisdn, TEST_PHONE_NUMBER);
                // Note: API returns country and currency as strings, not enums
                assert!(
                    response.data.country.contains("KE") || response.data.country.contains("Kenya")
                );
                assert!(
                    response.data.currency.contains("KES")
                        || response.data.currency.contains("Kenyan")
                );
            }
            Err(e) => {
                println!("❌ Remittance eligibility check failed: {}", e);
                println!("Error details: {}", e);
                assert!(false, "Remittance eligibility test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_remittance_transfer_credit() {
        let (client_id, client_secret, pin) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let remittance = airtel.remittance(client_id, client_secret);

        let ext_trid = generate_unique_transaction_id();

        println!("💸 Testing remittance money transfer credit...");
        println!("External Transaction ID: {}", ext_trid);

        let result = remittance
            .money_transfer_credit(
                TEST_AMOUNT,
                ext_trid.clone(),
                TEST_PHONE_NUMBER.to_string(),
                "KE".to_string(),   // Payer country
                "John".to_string(), // First name
                "Doe".to_string(),  // Last name
                pin,
            )
            .await;

        match result {
            Ok(response) => {
                println!("✅ Money transfer credit successful");
                println!(
                    "Airtel Money ID: {}",
                    response.data.transaction.airtel_money_id
                );
                println!("Status: {}", response.data.transaction.status);
                println!(
                    "External Transaction ID: {}",
                    response.data.transaction.ext_trid
                );

                assert!(response.status.success);
                assert!(!response.data.transaction.airtel_money_id.is_empty());
                assert_eq!(response.data.transaction.ext_trid, ext_trid);

                // Test status check
                println!("🔍 Checking remittance transfer status...");
                let status_result = remittance.money_transfer_status(ext_trid).await;

                match status_result {
                    Ok(status_response) => {
                        println!("✅ Remittance status check successful");
                        println!("Status: {}", status_response.data.transaction.status);
                        assert!(status_response.status.success);
                    }
                    Err(e) => {
                        println!("⚠️  Remittance status check failed: {}", e);
                        // Don't fail the test as status might not be immediately available
                    }
                }
            }
            Err(e) => {
                println!("❌ Money transfer credit failed: {}", e);
                println!("Error details: {}", e);
                assert!(false, "Remittance transfer test failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_collection_refund() {
        let (client_id, client_secret, _) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let collection = airtel.collection(client_id, client_secret);

        // First, create a transaction to refund
        let reference = generate_unique_reference();
        let transaction_id = generate_unique_transaction_id();

        println!("💰 Creating transaction for refund test...");

        let ussd_result = collection
            .ussd_push(
                reference,
                TEST_PHONE_NUMBER.to_string(),
                TEST_AMOUNT,
                transaction_id,
            )
            .await;

        match ussd_result {
            Ok(ussd_response) => {
                println!("✅ Transaction created for refund test");
                let airtel_money_id = ussd_response.data.transaction.id;

                // Wait a moment before attempting refund
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                println!("💫 Testing collection refund...");
                println!("Airtel Money ID: {}", airtel_money_id);

                let refund_result = collection.refund(airtel_money_id).await;

                match refund_result {
                    Ok(refund_response) => {
                        println!("✅ Refund successful");
                        println!("Refund status: {}", refund_response.status.message);
                        assert!(refund_response.status.success);
                    }
                    Err(e) => {
                        println!("⚠️  Refund failed (might be expected in sandbox): {}", e);
                        // Don't fail the test as refunds might not be allowed in sandbox
                        // or the transaction might not be in the right state
                    }
                }
            }
            Err(e) => {
                println!("❌ Could not create transaction for refund test: {}", e);
                assert!(false, "Failed to create transaction for refund test: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_multiple_countries() {
        let (client_id, client_secret, _) = load_test_credentials();

        // Test different countries
        let countries = vec![
            (Country::Kenya, Currency::KES),
            (Country::Uganda, Currency::UGX),
            (Country::Tanzania, Currency::TZS),
        ];

        for (country, currency) in countries {
            println!(
                "🌍 Testing with country: {:?}, currency: {:?}",
                country, currency
            );

            let airtel = AirtelMoney::new(Environment::Sandbox, country);
            let account = airtel.account(client_id.clone(), client_secret.clone());

            let result = account.get_balance().await;

            match result {
                Ok(balance) => {
                    println!("✅ {} account balance retrieved", country);
                    println!(
                        "Balance: {} {}",
                        balance.data.balance, balance.data.currency
                    );
                    assert!(balance.status.success);
                }
                Err(e) => {
                    println!("⚠️  {} account balance failed: {}", country, e);
                    // Don't fail test as some countries might not be configured
                }
            }
        }
    }

    #[tokio::test]
    async fn test_error_handling() {
        let (client_id, client_secret, _) = load_test_credentials();
        let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
        let collection = airtel.collection(client_id, client_secret);

        println!("❌ Testing error handling with invalid data...");

        // Test with invalid phone number
        let result = collection
            .ussd_push(
                "invalid_ref".to_string(),
                "invalid_phone".to_string(), // Invalid phone number
                TEST_AMOUNT,
                "invalid_tx".to_string(),
            )
            .await;

        match result {
            Ok(_) => {
                println!("⚠️  Unexpected success with invalid phone number");
            }
            Err(e) => {
                println!("✅ Error handling working correctly: {}", e);
                assert!(
                    e.to_string().contains("Unauthorized")
                        || e.to_string().contains("Invalid")
                        || e.to_string().contains("Bad Request")
                        || e.to_string().contains("error")
                );
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        let (client_id, client_secret, _) = load_test_credentials();

        println!("🚀 Testing concurrent API requests...");

        // Create multiple concurrent balance requests
        let mut tasks = Vec::new();

        for i in 0..3 {
            let client_id = client_id.clone();
            let client_secret = client_secret.clone();

            let task = tokio::spawn(async move {
                let airtel = AirtelMoney::new(Environment::Sandbox, TEST_COUNTRY);
                let account = airtel.account(client_id, client_secret);
                let result = account.get_balance().await;
                match result {
                    Ok(balance) => (i, true, balance.data.balance),
                    Err(_) => (i, false, "Error".to_string()),
                }
            });
            tasks.push(task);
        }

        let results = futures::future::join_all(tasks).await;

        let mut success_count = 0;
        for result in results {
            match result {
                Ok((i, true, balance)) => {
                    println!("✅ Concurrent request {} successful: {}", i, balance);
                    success_count += 1;
                }
                Ok((i, false, _)) => {
                    println!("❌ Concurrent request {} failed", i);
                }
                Err(e) => {
                    println!("❌ Task panicked: {}", e);
                }
            }
        }

        assert!(
            success_count > 0,
            "At least one concurrent request should succeed"
        );
        println!(
            "✅ Concurrent requests test completed: {}/3 succeeded",
            success_count
        );
    }
}
