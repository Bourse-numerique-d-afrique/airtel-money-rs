//! Example demonstrating how to use the Airtel Money callback server
//!
//! This example shows how to:
//! 1. Start the callback server
//! 2. Process incoming callback updates as a stream
//! 3. Handle different payment statuses
//!
//! To run this example:
//! ```bash
//! cargo run --example callback_server_usage --features callback_server
//! ```
//!
//! The server will start on http://127.0.0.1:8080 by default.
//! You can test it by sending a POST request to http://127.0.0.1:8080/webhooks/airtel

use airtel_rs::callback_server::{start_callback_server, CallbackServerConfig};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Airtel Money Callback Server Example");

    // Create server configuration (HTTP for development)
    let config = CallbackServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        webhook_secret: Some("your_webhook_secret".to_string()),
        cert_path: None, // No TLS for development
        key_path: None,
    };

    // For HTTPS/TLS in production, use:
    // let config = CallbackServerConfig {
    //     host: "0.0.0.0".to_string(),
    //     port: 443,
    //     webhook_secret: Some("your_webhook_secret".to_string()),
    //     cert_path: Some("path/to/cert.pem".to_string()),
    //     key_path: Some("path/to/key.pem".to_string()),
    // };

    // Or use the builder pattern:
    // let config = CallbackServerConfig::https_default()
    //     .with_tls("cert.pem".to_string(), "key.pem".to_string());

    // Start the callback server and get the stream of updates
    let callback_stream = start_callback_server(config).await?;
    futures_util::pin_mut!(callback_stream);

    println!("✅ Callback server started successfully!");
    println!("📡 Listening for callbacks...");
    println!("💡 You can test by sending POST requests to http://127.0.0.1:8080/webhooks/airtel");
    println!("📊 Health check available at http://127.0.0.1:8080/health");
    println!("🔄 Processing callbacks (Press Ctrl+C to stop)...\n");

    // Process incoming callbacks
    while let Some(update) = callback_stream.next().await {
        println!("📥 Received callback update:");
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

                // Here you would typically:
                // - Update your database with the successful payment
                // - Send confirmation emails/SMS to customer
                // - Update inventory if applicable
                // - Trigger fulfillment processes
                handle_successful_payment(&update).await;
            }
            "FAILED" => {
                println!("   ❌ Status: Payment failed");
                if let Some(ref message) = update.response.transaction.message {
                    println!("   📝 Reason: {}", message);
                }

                // Here you would typically:
                // - Update your database with the failed payment
                // - Send failure notifications to customer
                // - Reverse any provisional actions taken
                // - Log the failure for analysis
                handle_failed_payment(&update).await;
            }
            "PENDING" => {
                println!("   🔄 Status: Payment is pending");

                // Here you would typically:
                // - Keep the payment status as pending
                // - Set up monitoring for status changes
                // - Maybe notify customer about the pending status
                handle_pending_payment(&update).await;
            }
            status => {
                println!("   ❓ Status: Unknown status '{}'", status);

                // Log unknown status for investigation
                handle_unknown_status(&update, status).await;
            }
        }

        println!("   ✨ Callback processed successfully!\n");
    }

    Ok(())
}

/// Handle successful payment callbacks
async fn handle_successful_payment(_update: &airtel_rs::AirtelUpdates) {
    println!("🎉 Processing successful payment:");
    println!("    - Updating payment record in database...");
    println!("    - Sending confirmation email to customer...");
    println!("    - Updating inventory...");
    println!("    - Triggering order fulfillment...");

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("    ✅ Success processing completed!");
}

/// Handle failed payment callbacks
async fn handle_failed_payment(_update: &airtel_rs::AirtelUpdates) {
    println!("💥 Processing failed payment:");
    println!("    - Updating payment status to failed...");
    println!("    - Sending failure notification to customer...");
    println!("    - Reversing provisional actions...");
    println!("    - Logging failure for analysis...");

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    println!("    ✅ Failure processing completed!");
}

/// Handle pending payment callbacks
async fn handle_pending_payment(_update: &airtel_rs::AirtelUpdates) {
    println!("⏳ Processing pending payment:");
    println!("    - Maintaining pending status...");
    println!("    - Setting up status monitoring...");

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;

    println!("    ✅ Pending processing completed!");
}

/// Handle unknown status callbacks
async fn handle_unknown_status(_update: &airtel_rs::AirtelUpdates, status: &str) {
    println!("🤔 Processing unknown status '{}':", status);
    println!("    - Logging for investigation...");
    println!("    - Alerting development team...");

    // In a real application, you might want to:
    // - Log to a monitoring system
    // - Send alerts to developers
    // - Store the callback for manual review

    println!("    ⚠️  Unknown status logged for review!");
}
