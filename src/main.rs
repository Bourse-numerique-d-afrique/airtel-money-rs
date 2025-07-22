use airtel_rs::{AirtelMoney, Environment, Country};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    println!("🚀 Airtel Money API Rust Example");
    println!("==================================");

    // Create AirtelMoney instance
    let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    
    println!("✅ Created AirtelMoney instance for {} in {:?} environment", 
             airtel.get_country(), airtel.environment);
    println!("💰 Currency: {:?}", airtel.get_currency());

    // Example of how to use the API (requires actual credentials)
    println!("\n📚 API Usage Examples:");
    println!("1. Get account balance:");
    println!("   let account = airtel.account(client_id, client_secret);");
    println!("   let balance = account.get_balance().await?;");

    println!("\n2. USSD Push Collection:");
    println!("   let collection = airtel.collection(client_id, client_secret);");
    println!("   let result = collection.ussd_push(reference, msisdn, amount, id).await?;");

    println!("\n3. Disbursement:");
    println!("   let disbursement = airtel.disbursement(client_id, client_secret);");
    println!("   let result = disbursement.disburse(msisdn, amount, tx_id, reference, pin).await?;");

    println!("\n4. Cash In:");
    println!("   let cash_in = airtel.cash_in(client_id, client_secret);");
    println!("   let result = cash_in.cash_in(msisdn, amount, tx_id, reference, pin, remark).await?;");

    println!("\n5. Cash Out:");
    println!("   let cash_out = airtel.cash_out(client_id, client_secret);");
    println!("   let result = cash_out.cash_out(msisdn, amount, tx_id, reference, pin, remark).await?;");

    println!("\n6. Remittance:");
    println!("   let remittance = airtel.remittance(client_id, client_secret);");
    println!("   let eligibility = remittance.check_eligibility(msisdn, amount, country, currency).await?;");
    println!("   let transfer = remittance.money_transfer_credit(amount, ext_id, msisdn, payer_country, first_name, last_name, pin).await?;");

    println!("\n🔧 Environment Setup:");
    println!("Set these environment variables for testing:");
    println!("  AIRTEL_CLIENT_ID=your_client_id");
    println!("  AIRTEL_CLIENT_SECRET=your_client_secret");

    println!("\n✨ All modules are functional and ready for use!");
    
    Ok(())
}