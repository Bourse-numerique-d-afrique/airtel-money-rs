//! Example demonstrating TLS-enabled Airtel Money callback server
//!
//! This example shows how to:
//! 1. Start the callback server with TLS/HTTPS support
//! 2. Use TLS certificates for secure communication
//! 3. Process incoming callback updates over HTTPS
//!
//! To run this example:
//! ```bash
//! # First, generate certificates (for testing only):
//! openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
//!
//! # Then run the example:
//! cargo run --example callback_server_tls --features callback_server
//! ```
//!
//! The server will start on https://127.0.0.1:8443 by default.
//! You can test it by sending a POST request to https://127.0.0.1:8443/webhooks/airtel

use airtel_rs::callback_server::{start_callback_server, CallbackServerConfig};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Starting Airtel Money Callback Server with TLS");

    // Create TLS-enabled server configuration
    let config = CallbackServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8443, // HTTPS development port
        webhook_secret: Some("your_webhook_secret".to_string()),
        cert_path: Some("cert.pem".to_string()),
        key_path: Some("key.pem".to_string()),
    };

    // Alternative ways to create TLS configuration:

    // Using the https_default() method:
    // let config = CallbackServerConfig::https_default();

    // Using the builder pattern:
    // let config = CallbackServerConfig::default()
    //     .with_tls("cert.pem".to_string(), "key.pem".to_string());

    // Using environment variables:
    // export TLS_CERT_PATH=cert.pem
    // export TLS_KEY_PATH=key.pem
    // export CALLBACK_PORT=8443
    // let config = CallbackServerConfig::default();

    println!("📋 Configuration:");
    println!("   Host: {}", config.host);
    println!("   Port: {}", config.port);
    println!("   TLS Enabled: {}", config.is_tls_enabled());
    println!("   Cert Path: {:?}", config.cert_path);
    println!("   Key Path: {:?}", config.key_path);

    // Validate TLS files exist before starting server
    if let Some(cert_path) = &config.cert_path {
        if !std::path::Path::new(cert_path).exists() {
            eprintln!("❌ Certificate file not found: {}", cert_path);
            eprintln!("💡 Generate certificates with:");
            eprintln!("   openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes");
            return Err("Certificate file not found".into());
        }
    }

    if let Some(key_path) = &config.key_path {
        if !std::path::Path::new(key_path).exists() {
            eprintln!("❌ Private key file not found: {}", key_path);
            return Err("Private key file not found".into());
        }
    }

    // Start the callback server and get the stream of updates
    let callback_stream = start_callback_server(config).await?;
    futures_util::pin_mut!(callback_stream);

    println!("✅ TLS Callback server started successfully!");
    println!("📡 Listening for callbacks...");
    println!("💡 You can test by sending POST requests to https://127.0.0.1:8443/webhooks/airtel");
    println!("📊 Health check available at https://127.0.0.1:8443/health");
    println!("🔒 All communications are encrypted with TLS");
    println!("🔄 Processing callbacks (Press Ctrl+C to stop)...\n");

    // Process incoming callbacks
    while let Some(update) = callback_stream.next().await {
        println!("🔒 Received secure callback update:");
        println!("   Type: {:?}", update.callback_type);
        println!("   Remote Address: {}", update.remote_address);
        println!("   Transaction ID: {}", update.response.transaction.id);
        println!(
            "   Airtel Money ID: {}",
            update.response.transaction.airtel_money_id
        );
        println!(
            "   Amount: {} {}",
            update.response.amount, update.response.currency
        );

        if let Some(ref reference) = update.response.reference {
            println!("   Reference: {}", reference);
        }

        // Handle different payment statuses
        match update.response.transaction.status.as_str() {
            "SUCCESS" => {
                println!("   ✅ Status: Payment completed successfully!");
                handle_successful_payment(&update).await;
            }
            "FAILED" => {
                println!("   ❌ Status: Payment failed");
                if let Some(ref message) = update.response.transaction.message {
                    println!("   📝 Reason: {}", message);
                }
                handle_failed_payment(&update).await;
            }
            "PENDING" => {
                println!("   🔄 Status: Payment is pending");
                handle_pending_payment(&update).await;
            }
            status => {
                println!("   ❓ Status: Unknown status '{}'", status);
                handle_unknown_status(&update, status).await;
            }
        }

        println!("   ✨ Secure callback processed successfully!\n");
    }

    Ok(())
}

/// Handle successful payment callbacks with enhanced security logging
async fn handle_successful_payment(_update: &airtel_rs::AirtelUpdates) {
    println!("🎉 Processing successful payment over secure connection:");
    println!("    - Updating payment record in secure database...");
    println!("    - Sending encrypted confirmation to customer...");
    println!("    - Updating inventory with audit trail...");
    println!("    - Triggering secure order fulfillment...");

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("    ✅ Success processing completed securely!");
}

/// Handle failed payment callbacks with security considerations
async fn handle_failed_payment(_update: &airtel_rs::AirtelUpdates) {
    println!("💥 Processing failed payment over secure connection:");
    println!("    - Securely updating payment status to failed...");
    println!("    - Sending encrypted failure notification...");
    println!("    - Reversing actions with security audit...");
    println!("    - Logging failure in secure audit system...");

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    println!("    ✅ Failure processing completed securely!");
}

/// Handle pending payment callbacks
async fn handle_pending_payment(_update: &airtel_rs::AirtelUpdates) {
    println!("⏳ Processing pending payment over secure connection:");
    println!("    - Maintaining secure pending status...");
    println!("    - Setting up encrypted status monitoring...");

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;

    println!("    ✅ Pending processing completed securely!");
}

/// Handle unknown status callbacks with enhanced security logging
async fn handle_unknown_status(_update: &airtel_rs::AirtelUpdates, status: &str) {
    println!(
        "🤔 Processing unknown status '{}' over secure connection:",
        status
    );
    println!("    - Logging securely for investigation...");
    println!("    - Sending encrypted alert to development team...");
    println!("    - Storing in secure audit system for review...");

    println!("    ⚠️  Unknown status logged securely for review!");
}
