use airtel_rs::{AirtelMoney, Country, Currency, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Airtel Money client
    let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);

    // Replace with your actual credentials
    let client_id = "your_client_id".to_string();
    let client_secret = "your_client_secret".to_string();

    println!("Airtel Money Rust SDK Example");
    println!("============================");

    // Example 1: Account Balance
    println!("\n1. Getting Account Balance:");
    let account = airtel.account(client_id.clone(), client_secret.clone());
    match account.get_balance().await {
        Ok(balance) => {
            println!("✅ Balance retrieved: {}", balance.data.balance);
        }
        Err(e) => println!("❌ Error getting balance: {}", e),
    }

    // Example 2: USSD Push Collection
    println!("\n2. USSD Push Collection:");
    let collection = airtel.collection(client_id.clone(), client_secret.clone());
    match collection
        .ussd_push(
            "test_ref_123".to_string(),
            "254700000000".to_string(),
            100,
            "test_tx_123".to_string(),
        )
        .await
    {
        Ok(response) => {
            println!("✅ USSD Push initiated: {}", response.data.transaction.id);
        }
        Err(e) => println!("❌ Error in USSD push: {}", e),
    }

    // Example 3: Disbursement
    println!("\n3. Disbursement:");
    let disbursement = airtel.disbursement(client_id.clone(), client_secret.clone());
    match disbursement
        .disburse(
            "254700000000".to_string(),
            100,
            "test_tx_456".to_string(),
            "test_ref_456".to_string(),
            "1234".to_string(),
        )
        .await
    {
        Ok(response) => {
            println!(
                "✅ Disbursement successful: {}",
                response.data.transaction.id
            );
        }
        Err(e) => println!("❌ Error in disbursement: {}", e),
    }

    // Example 4: Cash In
    println!("\n4. Cash In:");
    let cash_in = airtel.cash_in(client_id.clone(), client_secret.clone());
    match cash_in
        .cash_in(
            "254700000000".to_string(),
            100,
            "test_tx_789".to_string(),
            "test_ref_789".to_string(),
            "1234".to_string(),
            "Payment for services".to_string(),
        )
        .await
    {
        Ok(response) => {
            println!("✅ Cash In successful: {}", response.data.transaction.id);
        }
        Err(e) => println!("❌ Error in cash in: {}", e),
    }

    // Example 5: Remittance Eligibility Check
    println!("\n5. Remittance Eligibility:");
    let remittance = airtel.remittance(client_id.clone(), client_secret.clone());
    match remittance
        .check_eligibility(
            "254700000000".to_string(),
            100,
            Country::Kenya,
            Currency::KES,
        )
        .await
    {
        Ok(response) => {
            println!(
                "✅ Eligibility check: {}",
                if response.data.eligible {
                    "Eligible"
                } else {
                    "Not eligible"
                }
            );
        }
        Err(e) => println!("❌ Error checking eligibility: {}", e),
    }

    println!("\n📝 Note: This example uses sandbox environment.");
    println!("To use with real credentials, set the environment variables:");
    println!("  export AIRTEL_CLIENT_ID=your_real_client_id");
    println!("  export AIRTEL_CLIENT_SECRET=your_real_client_secret");

    Ok(())
}
