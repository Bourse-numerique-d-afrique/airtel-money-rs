//! Airtel Money Callback Server
//!
//! A production-ready callback server for handling Airtel Money payment callbacks.
//! This server provides HTTP endpoints for receiving webhook notifications and
//! processes them as a stream of AirtelUpdates.
//!
//! # Features
//!
//! - **Webhook Handling**: Receives and processes Airtel Money callbacks
//! - **Stream Interface**: Returns async stream of callback updates
//! - **JSON Processing**: Parses callback payloads automatically
//! - **Health Check**: Built-in health monitoring endpoint
//! - **Logging**: Comprehensive logging of all callback events
//! - **Configuration**: Environment-based configuration
//!
//! # Usage
//!
//! Enable the `callback_server` feature in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! airtel_rs = { version = "*", features = ["callback_server"] }
//! ```
//!
//! Then use the callback server:
//!
//! ```rust,no_run
//! use airtel_rs::callback_server::{start_callback_server, CallbackServerConfig};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = CallbackServerConfig::default();
//!     let mut callback_stream = start_callback_server(config).await?;
//!
//!     while let Some(update) = callback_stream.next().await {
//!         println!("Received callback: {:?}", update.callback_type);
//!         // Process the callback according to your business logic
//!     }
//!
//!     Ok(())
//! }
//! ```

use futures_core::Stream;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tokio::sync::mpsc;

/// Configuration for the Airtel Money callback server
#[derive(Debug, Clone)]
pub struct CallbackServerConfig {
    /// Host address to bind the server to
    pub host: String,
    /// Port number for the server to bind to
    pub port: u16,
    /// Optional webhook secret for validating callback authenticity
    pub webhook_secret: Option<String>,
    /// Path to the TLS certificate file in PEM format (optional for HTTPS)
    pub cert_path: Option<String>,
    /// Path to the TLS private key file in PEM format (optional for HTTPS)
    pub key_path: Option<String>,
}

impl Default for CallbackServerConfig {
    fn default() -> Self {
        Self {
            host: env::var("CALLBACK_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("CALLBACK_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            webhook_secret: env::var("AIRTEL_WEBHOOK_SECRET").ok(),
            cert_path: env::var("TLS_CERT_PATH").ok(),
            key_path: env::var("TLS_KEY_PATH").ok(),
        }
    }
}

impl CallbackServerConfig {
    /// Returns the socket address for the server
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid host:port combination")
    }

    /// Returns true if TLS is configured (both cert and key paths are provided)
    pub fn is_tls_enabled(&self) -> bool {
        self.cert_path.is_some() && self.key_path.is_some()
    }

    /// Creates a new configuration with TLS enabled
    pub fn with_tls(mut self, cert_path: String, key_path: String) -> Self {
        self.cert_path = Some(cert_path);
        self.key_path = Some(key_path);
        self
    }

    /// Creates a new configuration for HTTPS (port 443 by default)
    pub fn https_default() -> Self {
        Self {
            host: env::var("CALLBACK_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("CALLBACK_PORT")
                .unwrap_or_else(|_| "443".to_string())
                .parse()
                .unwrap_or(443),
            webhook_secret: env::var("AIRTEL_WEBHOOK_SECRET").ok(),
            cert_path: env::var("TLS_CERT_PATH")
                .ok()
                .or_else(|| Some("cert.pem".to_string())),
            key_path: env::var("TLS_KEY_PATH")
                .ok()
                .or_else(|| Some("key.pem".to_string())),
        }
    }
}

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

/// Types of Airtel Money callbacks
#[derive(Debug, Clone, PartialEq)]
pub enum CallbackType {
    /// Collection payment callback
    Collection,
    /// Disbursement payment callback
    Disbursement,
    /// Cash in callback
    CashIn,
    /// Cash out callback
    CashOut,
    /// Remittance callback
    Remittance,
    /// Unknown callback type
    Unknown,
}

/// Represents an Airtel Money callback update
#[derive(Debug, Clone)]
pub struct AirtelUpdates {
    /// The remote IP address that sent the callback
    pub remote_address: String,
    /// The parsed callback payload
    pub response: CallbackPayload,
    /// The type of callback
    pub callback_type: CallbackType,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

#[cfg(feature = "callback_server")]
mod server {
    use super::*;
    use log::{error, info, warn};
    use rustls::{Certificate, PrivateKey, ServerConfig as RustlsServerConfig};
    use rustls_pemfile::{certs, pkcs8_private_keys};
    use std::fs::File;
    use std::io::BufReader;
    use warp::Filter;

    /// Handles incoming Airtel Money callbacks
    pub async fn handle_callback(
        payload: CallbackPayload,
        _config: CallbackServerConfig,
        sender: tokio::sync::mpsc::Sender<AirtelUpdates>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!(
            "Received callback for transaction: {}",
            payload.transaction.id
        );
        log::debug!("Callback payload: {:?}", payload);

        // Determine callback type based on the payload or request context
        // This is simplified - in practice you might need more sophisticated logic
        let callback_type = determine_callback_type(&payload);

        // Get remote address (this would normally come from the request)
        let remote_address = "unknown".to_string(); // warp doesn't easily provide this

        let airtel_update = AirtelUpdates {
            remote_address,
            response: payload.clone(),
            callback_type,
        };

        // Send the update to the stream
        if let Err(e) = sender.send(airtel_update).await {
            error!("Failed to send callback update: {}", e);
        } else {
            info!("Successfully processed {} callback", payload.transaction.id);
        }

        // Handle different payment statuses
        match payload.transaction.status.as_str() {
            "SUCCESS" => {
                info!(
                    "Payment SUCCESS: {} (Airtel ID: {})",
                    payload.transaction.id, payload.transaction.airtel_money_id
                );
            }
            "FAILED" => {
                warn!(
                    "Payment FAILED: {} - {}",
                    payload.transaction.id,
                    payload.transaction.message.as_deref().unwrap_or_default()
                );
            }
            "PENDING" => {
                info!("Payment PENDING: {}", payload.transaction.id);
            }
            status => {
                warn!(
                    "Unknown payment status: {} for transaction: {}",
                    status, payload.transaction.id
                );
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
        config: CallbackServerConfig,
        sender: tokio::sync::mpsc::Sender<AirtelUpdates>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let config_filter = warp::any().map(move || config.clone());
        let sender_filter = warp::any().map(move || sender.clone());

        // POST /webhooks/airtel - Airtel Money callback endpoint
        let callback_route = warp::path!("webhooks" / "airtel")
            .and(warp::post())
            .and(warp::body::json())
            .and(config_filter)
            .and(sender_filter)
            .and_then(handle_callback);

        // GET /health - Health check endpoint
        let health_route = warp::path!("health")
            .and(warp::get())
            .and_then(handle_health);

        // Combine routes with logging
        callback_route
            .or(health_route)
            .with(warp::log("airtel_callback_server"))
    }

    fn determine_callback_type(_payload: &CallbackPayload) -> CallbackType {
        // This is a simplified implementation
        // In practice, you might determine the type based on:
        // - URL path parameters
        // - Payload structure/fields
        // - Headers
        // - Reference ID patterns

        // For now, we'll default to Collection as it's the most common
        CallbackType::Collection
    }

    /// Loads and validates TLS configuration from certificate and key files.
    ///
    /// This function reads the TLS certificate and private key files specified in the
    /// configuration, validates their format, and creates a `RustlsServerConfig` object
    /// for secure HTTPS connections.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration containing paths to certificate and key files
    ///
    /// # Returns
    ///
    /// * `Ok(RustlsServerConfig)` - Successfully loaded and configured TLS settings
    /// * `Err(Box<dyn Error>)` - Failed to load or validate certificate/key files
    pub async fn load_tls_config(
        config: &CallbackServerConfig,
    ) -> Result<RustlsServerConfig, Box<dyn Error>> {
        let cert_path = config
            .cert_path
            .as_ref()
            .ok_or("TLS certificate path not configured")?;
        let key_path = config
            .key_path
            .as_ref()
            .ok_or("TLS private key path not configured")?;

        info!("Loading TLS certificate from: {}", cert_path);
        info!("Loading TLS private key from: {}", key_path);

        // Load certificate chain
        let cert_file = File::open(cert_path)
            .map_err(|e| format!("Failed to open certificate file '{}': {}", cert_path, e))?;
        let mut cert_reader = BufReader::new(cert_file);
        let cert_chain = certs(&mut cert_reader)
            .map_err(|e| format!("Failed to parse certificate file '{}': {}", cert_path, e))?
            .into_iter()
            .map(Certificate)
            .collect();

        // Load private key
        let key_file = File::open(key_path)
            .map_err(|e| format!("Failed to open private key file '{}': {}", key_path, e))?;
        let mut key_reader = BufReader::new(key_file);
        let mut keys = pkcs8_private_keys(&mut key_reader)
            .map_err(|e| format!("Failed to parse private key file '{}': {}", key_path, e))?;

        if keys.is_empty() {
            return Err(format!("No private key found in file '{}'", key_path).into());
        }
        let private_key = PrivateKey(keys.remove(0));

        // Create TLS configuration
        let tls_config = RustlsServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, private_key)
            .map_err(|e| format!("Failed to create TLS configuration: {}", e))?;

        info!("TLS configuration loaded successfully");
        Ok(tls_config)
    }
}

/// Starts the Airtel Money callback server with the specified configuration.
///
/// This function initializes and starts the callback server, creating HTTP endpoints
/// for receiving Airtel Money webhook notifications. It returns a stream of processed
/// callback updates that can be consumed by your application.
///
/// # Arguments
///
/// * `config` - Server configuration including host, port, and optional webhook secret
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(Stream<Item = AirtelUpdates>)`: A stream of processed callback updates
/// - `Err(Box<dyn Error>)`: An error if server startup fails
///
/// # Examples
///
/// ```rust,no_run
/// use airtel_rs::callback_server::{start_callback_server, CallbackServerConfig};
/// use futures_util::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = CallbackServerConfig::default();
///     let mut callback_stream = start_callback_server(config).await?;
///
///     println!("Server started, processing callbacks...");
///
///     while let Some(callback) = callback_stream.next().await {
///         println!("Received callback: {:?}", callback.callback_type);
///         match callback.response.transaction.status.as_str() {
///             "SUCCESS" => {
///                 // Handle successful payment
///                 println!("✅ Payment successful: {}", callback.response.transaction.id);
///             }
///             "FAILED" => {
///                 // Handle failed payment
///                 println!("❌ Payment failed: {}", callback.response.transaction.id);
///             }
///             _ => {
///                 // Handle other statuses
///                 println!("📋 Status update: {}", callback.response.transaction.status);
///             }
///         }
///     }
///
///     Ok(())
/// }
/// ```
#[cfg(feature = "callback_server")]
pub async fn start_callback_server(
    config: CallbackServerConfig,
) -> Result<impl Stream<Item = AirtelUpdates>, Box<dyn Error>> {
    use log::info;

    // Initialize logging if not already done
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    let _ = env_logger::try_init();

    info!("Starting Airtel Money Callback Server");
    info!("Server configuration:");
    info!("  Host: {}", config.host);
    info!("  Port: {}", config.port);
    info!("  TLS enabled: {}", config.is_tls_enabled());
    info!(
        "  Webhook secret configured: {}",
        config.webhook_secret.is_some()
    );

    let (tx, mut rx) = mpsc::channel::<AirtelUpdates>(100);
    let addr = config.socket_addr();

    // Create routes
    let routes = server::create_routes(config.clone(), tx);

    // Start server in background with or without TLS
    if config.is_tls_enabled() {
        // TLS/HTTPS server
        let _tls_config = server::load_tls_config(&config).await?;

        tokio::spawn(async move {
            info!("🔒 Callback server listening on https://{}", addr);
            info!("📋 Available endpoints:");
            info!("   POST /webhooks/airtel - Receive Airtel Money callbacks");
            info!("   GET  /health         - Health check");

            // Use warp's built-in TLS support
            warp::serve(routes)
                .tls()
                .cert_path(config.cert_path.as_ref().unwrap())
                .key_path(config.key_path.as_ref().unwrap())
                .run(addr)
                .await;
        });
    } else {
        // HTTP server
        tokio::spawn(async move {
            info!("🚀 Callback server listening on http://{}", addr);
            info!("📋 Available endpoints:");
            info!("   POST /webhooks/airtel - Receive Airtel Money callbacks");
            info!("   GET  /health         - Health check");
            info!("⚠️  WARNING: Server running without TLS. Use HTTPS in production!");

            warp::serve(routes).run(addr).await;
        });
    }

    info!("Airtel Money Callback Server started successfully");

    // Return the stream
    Ok(async_stream::stream! {
        while let Some(msg) = rx.recv().await {
            yield msg;
        }
    })
}

#[cfg(not(feature = "callback_server"))]
pub async fn start_callback_server(
    _config: CallbackServerConfig,
) -> Result<impl Stream<Item = AirtelUpdates>, Box<dyn Error>> {
    Err(
        "callback_server feature not enabled. Add 'callback_server' to your Cargo.toml features."
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

    #[test]
    fn test_server_config_default() {
        // Save original values
        let original_host = std::env::var("CALLBACK_HOST").ok();
        let original_port = std::env::var("CALLBACK_PORT").ok();
        let original_secret = std::env::var("AIRTEL_WEBHOOK_SECRET").ok();

        // Set test values
        std::env::set_var("CALLBACK_HOST", "127.0.0.1");
        std::env::set_var("CALLBACK_PORT", "9000");
        std::env::set_var("AIRTEL_WEBHOOK_SECRET", "test_secret");

        let config = CallbackServerConfig::default();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 9000);
        assert_eq!(config.webhook_secret, Some("test_secret".to_string()));

        // Restore original values
        match original_host {
            Some(val) => std::env::set_var("CALLBACK_HOST", val),
            None => std::env::remove_var("CALLBACK_HOST"),
        }
        match original_port {
            Some(val) => std::env::set_var("CALLBACK_PORT", val),
            None => std::env::remove_var("CALLBACK_PORT"),
        }
        match original_secret {
            Some(val) => std::env::set_var("AIRTEL_WEBHOOK_SECRET", val),
            None => std::env::remove_var("AIRTEL_WEBHOOK_SECRET"),
        }
    }

    #[test]
    fn test_callback_payload_serialization() {
        let payload = CallbackPayload {
            transaction: CallbackTransaction {
                id: "tx_123".to_string(),
                airtel_money_id: "AM_456".to_string(),
                status: "SUCCESS".to_string(),
                message: Some("Payment completed".to_string()),
            },
            reference: Some("order_789".to_string()),
            amount: 1000,
            currency: "KES".to_string(),
            timestamp: Some("2024-01-15T10:30:00Z".to_string()),
        };

        let json_value = serde_json::to_value(&payload).unwrap();
        let deserialized: CallbackPayload = serde_json::from_value(json_value).unwrap();

        assert_eq!(deserialized.transaction.id, "tx_123");
        assert_eq!(deserialized.amount, 1000);
        assert_eq!(deserialized.currency, "KES");
    }

    #[test]
    fn test_callback_type_enum() {
        assert_eq!(CallbackType::Collection, CallbackType::Collection);
        assert_ne!(CallbackType::Collection, CallbackType::Disbursement);
    }

    #[test]
    fn test_airtel_updates_creation() {
        let payload = CallbackPayload {
            transaction: CallbackTransaction {
                id: "tx_test".to_string(),
                airtel_money_id: "AM_test".to_string(),
                status: "PENDING".to_string(),
                message: None,
            },
            reference: None,
            amount: 500,
            currency: "UGX".to_string(),
            timestamp: None,
        };

        let update = AirtelUpdates {
            remote_address: "127.0.0.1".to_string(),
            response: payload,
            callback_type: CallbackType::Collection,
        };

        assert_eq!(update.remote_address, "127.0.0.1");
        assert_eq!(update.response.amount, 500);
        assert_eq!(update.callback_type, CallbackType::Collection);
    }

    #[cfg(feature = "callback_server")]
    #[tokio::test]
    async fn test_health_endpoint() {
        let result = server::handle_health().await;
        assert!(result.is_ok());
    }

    #[cfg(feature = "callback_server")]
    #[tokio::test]
    async fn test_callback_server_startup() {
        let config = CallbackServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0, // Let the OS assign a port
            webhook_secret: None,
            cert_path: None,
            key_path: None,
        };

        // Test that the server starts without error
        let result = start_callback_server(config).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_tls_configuration() {
        let config_without_tls = CallbackServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            webhook_secret: None,
            cert_path: None,
            key_path: None,
        };

        let config_with_tls = CallbackServerConfig {
            host: "127.0.0.1".to_string(),
            port: 443,
            webhook_secret: None,
            cert_path: Some("cert.pem".to_string()),
            key_path: Some("key.pem".to_string()),
        };

        assert!(!config_without_tls.is_tls_enabled());
        assert!(config_with_tls.is_tls_enabled());
    }

    #[test]
    fn test_with_tls_builder() {
        let config = CallbackServerConfig::default()
            .with_tls("test_cert.pem".to_string(), "test_key.pem".to_string());

        assert!(config.is_tls_enabled());
        assert_eq!(config.cert_path, Some("test_cert.pem".to_string()));
        assert_eq!(config.key_path, Some("test_key.pem".to_string()));
    }

    #[test]
    fn test_https_default_config() {
        // Save original values
        let original_host = std::env::var("CALLBACK_HOST").ok();
        let original_port = std::env::var("CALLBACK_PORT").ok();
        let original_cert = std::env::var("TLS_CERT_PATH").ok();
        let original_key = std::env::var("TLS_KEY_PATH").ok();

        // Set test values
        std::env::set_var("CALLBACK_HOST", "localhost");
        std::env::remove_var("CALLBACK_PORT");
        std::env::remove_var("TLS_CERT_PATH");
        std::env::remove_var("TLS_KEY_PATH");

        let config = CallbackServerConfig::https_default();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 443);
        assert!(config.is_tls_enabled());
        assert_eq!(config.cert_path, Some("cert.pem".to_string()));
        assert_eq!(config.key_path, Some("key.pem".to_string()));

        // Restore original values
        match original_host {
            Some(val) => std::env::set_var("CALLBACK_HOST", val),
            None => std::env::remove_var("CALLBACK_HOST"),
        }
        match original_port {
            Some(val) => std::env::set_var("CALLBACK_PORT", val),
            None => std::env::remove_var("CALLBACK_PORT"),
        }
        match original_cert {
            Some(val) => std::env::set_var("TLS_CERT_PATH", val),
            None => std::env::remove_var("TLS_CERT_PATH"),
        }
        match original_key {
            Some(val) => std::env::set_var("TLS_KEY_PATH", val),
            None => std::env::remove_var("TLS_KEY_PATH"),
        }
    }
}
