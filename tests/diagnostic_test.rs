use airtel_rs::{AirtelMoney, Environment, Country, Currency};
use std::env;
use dotenv::dotenv;

#[tokio::test]
async fn test_credentials_and_environment() {
    dotenv().ok();
    
    println!("🔍 Airtel Money API Diagnostic Test");
    println!("===================================");
    
    // Load and display credentials (masked)
    let client_id = env::var("AIRTEL_CLIENT_ID").expect("AIRTEL_CLIENT_ID must be set");
    let client_secret = env::var("AIRTEL_CLIENT_SECRET").expect("AIRTEL_CLIENT_SECRET must be set");
    let pin = env::var("AIRTEL_PIN").unwrap_or_default();
    
    println!("📋 Configuration:");
    println!("Client ID: {}...", &client_id[..8]);
    println!("Client Secret: {}...", &client_secret[..8]);
    println!("PIN configured: {}", if pin.is_empty() { "No" } else { "Yes" });
    
    // Test both environments
    let environments = vec![
        ("Sandbox", Environment::Sandbox),
        ("Production", Environment::Production),
    ];
    
    for (env_name, environment) in environments {
        println!("\n🔧 Testing {} Environment:", env_name);
        println!("{}", "─".repeat(40));
        
        let airtel = AirtelMoney::new(environment, Country::Kenya);
        let account = airtel.account(client_id.clone(), client_secret.clone());
        
        match account.get_balance().await {
            Ok(balance) => {
                println!("✅ {} - SUCCESS", env_name);
                println!("   Balance: {}", balance.data.balance);
                println!("   Currency: {:?}", balance.data.currency);
                println!("   Account Status: {}", balance.data.account_status);
                println!("   API Response Code: {}", balance.status.code);
                println!("   API Message: {}", balance.status.message);
                
                // Test other countries if balance works
                println!("\n🌍 Testing other countries in {} environment:", env_name);
                let countries = vec![
                    (Country::Uganda, Currency::UGX),
                    (Country::Tanzania, Currency::TZS),
                    (Country::Nigeria, Currency::NGN),
                ];
                
                for (country, _currency) in countries {
                    let airtel_country = AirtelMoney::new(environment, country);
                    let account_country = airtel_country.account(client_id.clone(), client_secret.clone());
                    
                    match account_country.get_balance().await {
                        Ok(bal) => {
                            println!("   ✅ {}: {} {}", country, bal.data.balance, bal.data.currency);
                        }
                        Err(e) => {
                            println!("   ❌ {}: {}", country, e);
                        }
                    }
                }
                break; // Stop after first successful environment
            }
            Err(e) => {
                println!("❌ {} - FAILED", env_name);
                println!("   Error: {}", e);
                
                // Analyze the error
                if e.to_string().contains("Unauthorized") {
                    println!("   💡 Possible causes:");
                    println!("      - Credentials are for a different environment");
                    println!("      - Client ID/Secret are incorrect");
                    println!("      - Account may be suspended or inactive");
                    println!("      - Additional authentication steps required");
                } else if e.to_string().contains("timeout") || e.to_string().contains("connection") {
                    println!("   💡 Network connectivity issue");
                } else {
                    println!("   💡 Other API error - check Airtel Money documentation");
                }
            }
        }
    }
    
    println!("\n📝 Next Steps:");
    println!("1. If both environments fail, verify credentials with Airtel Money support");
    println!("2. Check if your account requires additional setup (KYC, activation, etc.)");
    println!("3. Verify that the sandbox/production environment matches your credentials");
    println!("4. Confirm that your account has API access enabled");
    
    println!("\n🔗 API Endpoints being tested:");
    println!("Sandbox: https://openapi.airtel.africa/auth/oauth2/token");
    println!("Production: https://openapi.airtel.africa/auth/oauth2/token");
    
    println!("\n✅ Diagnostic test completed!");
}