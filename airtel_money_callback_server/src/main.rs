//! Airtel Money Callback Server
//!
//! A standalone HTTP server for receiving Airtel Money payment callbacks.
//! This server handles webhook notifications from Airtel Money when transaction
//! statuses change, allowing your application to respond to payment events in real-time.
//!
//! # Features
//!
//! - **Webhook Handling**: Receives and processes Airtel Money callbacks
//! - **Signature Verification**: Validates callback authenticity (when implemented)
//! - **JSON Processing**: Parses callback payloads automatically
//! - **Logging**: Comprehensive logging of all callback events
//! - **Configuration**: Environment-based configuration
//!
//! # Usage
//!
//! ```bash
//! # Set required environment variables
//! export CALLBACK_PORT=8080
//! export CALLBACK_HOST=0.0.0.0
//! export AIRTEL_WEBHOOK_SECRET=your_webhook_secret
//!
//! # Run the callback server
//! cargo run
//! ```
//!
//! # Environment Variables
//!
//! - `CALLBACK_PORT` - Port to listen on (default: 8080)
//! - `CALLBACK_HOST` - Host/IP to bind to (default: 0.0.0.0)
//! - `AIRTEL_WEBHOOK_SECRET` - Secret for validating webhook signatures
//! - `RUST_LOG` - Log level (default: info)
//!
//! # Callback Endpoints
//!
//! - `POST /webhooks/airtel` - Receives Airtel Money callbacks
//! - `GET /health` - Health check endpoint
//!
//! # Example Callback Payload
//!
//! ```json
//! {
//!   "transaction": {
//!     "id": "your_transaction_id",
//!     "airtel_money_id": "AM123456789",
//!     "status": "SUCCESS"
//!   },
//!   "reference": "your_reference_id",
//!   "amount": 1000,
//!   "currency": "KES",
//!   "timestamp": "2024-01-15T10:30:00Z"
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use warp::Filter;

/// Callback payload structure for Airtel Money notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackPayload {
    /// Transaction information
    pub transaction: CallbackTransaction,
    /// Merchant reference from original request
    pub reference: Option<String>,
    /// Transaction amount in smallest currency unit
    pub amount: i32,
    /// Currency code (e.g., "KES", "UGX")
    pub currency: String,
    /// Timestamp of the callback
    pub timestamp: Option<String>,
}

/// Transaction details in callback payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackTransaction {
    /// Client-provided transaction ID
    pub id: String,
    /// Airtel Money system transaction ID
    pub airtel_money_id: String,
    /// Current transaction status
    pub status: String,
    /// Optional status description
    pub message: Option<String>,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

/// Configuration for the callback server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub webhook_secret: Option<String>,
}

impl ServerConfig {
    /// Creates server configuration from environment variables
    pub fn from_env() -> Self {
        let host = env::var("CALLBACK_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("CALLBACK_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("CALLBACK_PORT must be a valid port number");
        let webhook_secret = env::var("AIRTEL_WEBHOOK_SECRET").ok();

        Self {
            host,
            port,
            webhook_secret,
        }
    }

    /// Returns the socket address for the server
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid host:port combination")
    }
}

/// Handles incoming Airtel Money callbacks
pub async fn handle_callback(
    payload: CallbackPayload,
    _config: ServerConfig,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Received callback for transaction: {}", payload.transaction.id);
    log::debug!("Callback payload: {:?}", payload);

    // Here you would typically:
    // 1. Validate the callback signature (if webhook_secret is provided)
    // 2. Update your database with the transaction status
    // 3. Trigger any business logic based on the status
    // 4. Send notifications to relevant parties

    match payload.transaction.status.as_str() {
        "SUCCESS" => {
            log::info!(
                "Payment SUCCESS: {} (Airtel ID: {})",
                payload.transaction.id,
                payload.transaction.airtel_money_id
            );
            // Handle successful payment
            handle_successful_payment(&payload).await;
        }
        "FAILED" => {
            log::warn!(
                "Payment FAILED: {} - {}",
                payload.transaction.id,
                payload.transaction.message.as_deref().unwrap_or_default()
            );
            // Handle failed payment
            handle_failed_payment(&payload).await;
        }
        "PENDING" => {
            log::info!("Payment PENDING: {}", payload.transaction.id);
            // Handle pending payment (usually no action needed)
        }
        status => {
            log::warn!("Unknown payment status: {} for transaction: {}", status, payload.transaction.id);
        }
    }

    // Return success response
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "status": "ok",
            "message": "Callback processed successfully"
        })),
        warp::http::StatusCode::OK,
    ))
}

/// Handles successful payment callbacks
async fn handle_successful_payment(payload: &CallbackPayload) {
    log::info!("Processing successful payment: {}", payload.transaction.id);
    
    // Example: Update database, send confirmation email, etc.
    // In a real application, you would:
    // 1. Update your order/payment record in the database
    // 2. Send confirmation emails/SMS to customer
    // 3. Update inventory if applicable
    // 4. Trigger fulfillment processes
    
    // Placeholder for your business logic
    println!("✅ Payment completed successfully!");
    println!("   Transaction ID: {}", payload.transaction.id);
    println!("   Airtel Money ID: {}", payload.transaction.airtel_money_id);
    println!("   Amount: {} {}", payload.amount, payload.currency);
    if let Some(ref reference) = payload.reference {
        println!("   Reference: {}", reference);
    }
}

/// Handles failed payment callbacks
async fn handle_failed_payment(payload: &CallbackPayload) {
    log::warn!("Processing failed payment: {}", payload.transaction.id);
    
    // Example: Update database, send failure notification, etc.
    // In a real application, you would:
    // 1. Update your order/payment record as failed
    // 2. Send failure notifications to customer
    // 3. Reverse any provisional actions taken
    // 4. Log the failure for analysis
    
    // Placeholder for your business logic
    println!("❌ Payment failed!");
    println!("   Transaction ID: {}", payload.transaction.id);
    println!("   Reason: {}", payload.transaction.message.as_deref().unwrap_or("Unknown"));
    if let Some(ref reference) = payload.reference {
        println!("   Reference: {}", reference);
    }
}

/// Health check endpoint handler
pub async fn handle_health() -> Result<impl warp::Reply, warp::Rejection> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(warp::reply::json(&response))
}

/// Creates the warp filter for callback endpoints
pub fn create_routes(
    config: ServerConfig,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let config = warp::any().map(move || config.clone());

    // POST /webhooks/airtel - Airtel Money callback endpoint
    let callback_route = warp::path!("webhooks" / "airtel")
        .and(warp::post())
        .and(warp::body::json())
        .and(config)
        .and_then(handle_callback);

    // GET /health - Health check endpoint
    let health_route = warp::path!("health")
        .and(warp::get())
        .and_then(handle_health);

    // Combine routes with logging
    callback_route
        .or(health_route)
        .with(warp::log("callback_server"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize environment variables if .env file exists
    if let Err(_) = dotenv::dotenv() {
        // .env file doesn't exist, which is fine
    }

    // Initialize logging
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // Load configuration
    let config = ServerConfig::from_env();
    let addr = config.socket_addr();

    log::info!("Starting Airtel Money Callback Server");
    log::info!("Server configuration:");
    log::info!("  Host: {}", config.host);
    log::info!("  Port: {}", config.port);
    log::info!("  Webhook secret configured: {}", config.webhook_secret.is_some());

    // Create routes
    let routes = create_routes(config);

    // Start server
    log::info!("🚀 Callback server listening on http://{}", addr);
    log::info!("📋 Available endpoints:");
    log::info!("   POST /webhooks/airtel - Receive Airtel Money callbacks");
    log::info!("   GET  /health         - Health check");

    warp::serve(routes).run(addr).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_server_config_from_env() {
        env::set_var("CALLBACK_HOST", "127.0.0.1");
        env::set_var("CALLBACK_PORT", "9000");
        env::set_var("AIRTEL_WEBHOOK_SECRET", "test_secret");

        let config = ServerConfig::from_env();
        
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 9000);
        assert_eq!(config.webhook_secret, Some("test_secret".to_string()));
    }

    #[test]
    fn test_callback_payload_deserialization() {
        let json_payload = json!({
            "transaction": {
                "id": "tx_123",
                "airtel_money_id": "AM_456",
                "status": "SUCCESS",
                "message": "Payment completed"
            },
            "reference": "order_789",
            "amount": 1000,
            "currency": "KES",
            "timestamp": "2024-01-15T10:30:00Z"
        });

        let payload: CallbackPayload = serde_json::from_value(json_payload).unwrap();
        
        assert_eq!(payload.transaction.id, "tx_123");
        assert_eq!(payload.transaction.airtel_money_id, "AM_456");
        assert_eq!(payload.transaction.status, "SUCCESS");
        assert_eq!(payload.reference, Some("order_789".to_string()));
        assert_eq!(payload.amount, 1000);
        assert_eq!(payload.currency, "KES");
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let result = handle_health().await;
        assert!(result.is_ok());
    }
}
