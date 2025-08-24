# Airtel Money Callback Server

This document explains how to use the Airtel Money callback server feature that processes webhook notifications as a stream of updates.

## Overview

The callback server provides:
- **Stream Interface**: Returns async stream of `AirtelUpdates` for processing
- **Webhook Handling**: HTTP endpoints for receiving Airtel Money callbacks
- **Health Monitoring**: Built-in health check endpoint
- **Configuration**: Environment-based configuration
- **Type Safety**: Strongly typed callback structures

## Setup

### 1. Enable the Feature

Add the `callback_server` feature to your `Cargo.toml`:

```toml
[dependencies]
airtel_rs = { version = "*", features = ["callback_server"] }
```

### 2. Basic Usage

```rust
use airtel_rs::callback_server::{start_callback_server, CallbackServerConfig};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the server
    let config = CallbackServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        webhook_secret: Some("your_webhook_secret".to_string()),
    };

    // Start the callback server
    let callback_stream = start_callback_server(config).await?;
    futures_util::pin_mut!(callback_stream);

    // Process callbacks as they arrive
    while let Some(update) = callback_stream.next().await {
        println!("Received callback: {:?}", update.callback_type);
        
        match update.response.transaction.status.as_str() {
            "SUCCESS" => {
                // Handle successful payment
                process_successful_payment(&update).await;
            }
            "FAILED" => {
                // Handle failed payment
                process_failed_payment(&update).await;
            }
            "PENDING" => {
                // Handle pending payment
                process_pending_payment(&update).await;
            }
            _ => {
                // Handle unknown status
                println!("Unknown status: {}", update.response.transaction.status);
            }
        }
    }

    Ok(())
}
```

## Configuration

### Environment Variables

The server can be configured using environment variables:

```bash
export CALLBACK_HOST=0.0.0.0        # Host to bind to (default: 0.0.0.0)
export CALLBACK_PORT=8080            # Port to listen on (default: 8080)
export AIRTEL_WEBHOOK_SECRET=secret  # Optional webhook secret for validation

# TLS/HTTPS Configuration (optional)
export TLS_CERT_PATH=/path/to/cert.pem  # Path to TLS certificate
export TLS_KEY_PATH=/path/to/key.pem    # Path to TLS private key
```

### Programmatic Configuration

#### HTTP Configuration (Development)

```rust
use airtel_rs::CallbackServerConfig;

let config = CallbackServerConfig {
    host: "127.0.0.1".to_string(),
    port: 8080,
    webhook_secret: Some("your_webhook_secret".to_string()),
    cert_path: None,
    key_path: None,
};
```

#### HTTPS/TLS Configuration (Production)

```rust
use airtel_rs::CallbackServerConfig;

// Explicit configuration
let config = CallbackServerConfig {
    host: "0.0.0.0".to_string(),
    port: 443,
    webhook_secret: Some("your_webhook_secret".to_string()),
    cert_path: Some("/path/to/cert.pem".to_string()),
    key_path: Some("/path/to/key.pem".to_string()),
};

// Using builder pattern
let config = CallbackServerConfig::default()
    .with_tls("cert.pem".to_string(), "key.pem".to_string());

// Using HTTPS defaults
let config = CallbackServerConfig::https_default();
```

### Default Configuration

```rust
// Uses environment variables with fallbacks
let config = CallbackServerConfig::default();
```

## Endpoints

The callback server exposes these HTTP endpoints:

- **POST /webhooks/airtel** - Receives Airtel Money callbacks
- **GET /health** - Health check endpoint

## Data Types

### AirtelUpdates

The main structure containing callback information:

```rust
pub struct AirtelUpdates {
    pub remote_address: String,      // IP address of the callback sender
    pub response: CallbackPayload,   // Parsed callback payload
    pub callback_type: CallbackType, // Type of callback
}
```

### CallbackPayload

The callback data from Airtel Money:

```rust
pub struct CallbackPayload {
    pub transaction: CallbackTransaction,
    pub reference: Option<String>,    // Merchant reference
    pub amount: i32,                 // Amount in smallest currency unit
    pub currency: String,            // Currency code (KES, UGX, etc.)
    pub timestamp: Option<String>,   // Callback timestamp
}
```

### CallbackTransaction

Transaction details:

```rust
pub struct CallbackTransaction {
    pub id: String,                  // Your transaction ID
    pub airtel_money_id: String,    // Airtel Money transaction ID
    pub status: String,             // Transaction status
    pub message: Option<String>,    // Optional status message
}
```

### CallbackType

Enum representing different callback types:

```rust
pub enum CallbackType {
    Collection,    // Payment collection
    Disbursement,  // Money disbursement
    CashIn,       // Cash deposit
    CashOut,      // Cash withdrawal
    Remittance,   // Cross-border transfer
    Unknown,      // Unknown callback type
}
```

## Example Implementation

### Processing Different Payment Statuses

```rust
async fn process_callback(update: &AirtelUpdates) {
    match update.response.transaction.status.as_str() {
        "SUCCESS" => {
            // Update database
            database::mark_payment_successful(&update.response.transaction.id).await;
            
            // Send confirmation
            email::send_payment_confirmation(
                &update.response.reference.as_ref().unwrap_or(&"N/A".to_string()),
                update.response.amount,
                &update.response.currency
            ).await;
            
            // Process order fulfillment
            orders::fulfill_order(&update.response.transaction.id).await;
        }
        "FAILED" => {
            // Update database
            database::mark_payment_failed(&update.response.transaction.id).await;
            
            // Send failure notification
            email::send_payment_failure_notification(
                &update.response.reference.as_ref().unwrap_or(&"N/A".to_string()),
                &update.response.transaction.message.as_ref().unwrap_or(&"Unknown error".to_string())
            ).await;
        }
        "PENDING" => {
            // Keep monitoring
            monitoring::schedule_payment_check(&update.response.transaction.id).await;
        }
        _ => {
            // Log unknown status
            log::warn!("Unknown payment status: {}", update.response.transaction.status);
        }
    }
}
```

### Database Integration

```rust
async fn handle_payment_callback(update: &AirtelUpdates) -> Result<(), DatabaseError> {
    let payment = Payment {
        id: update.response.transaction.id.clone(),
        airtel_money_id: update.response.transaction.airtel_money_id.clone(),
        status: update.response.transaction.status.clone(),
        amount: update.response.amount,
        currency: update.response.currency.clone(),
        reference: update.response.reference.clone(),
        updated_at: chrono::Utc::now(),
    };
    
    database::upsert_payment(payment).await?;
    Ok(())
}
```

## Testing

### Unit Tests

Run the callback server tests:

```bash
cargo test callback_server --features callback_server
```

### Integration Testing

You can test the callback server by sending HTTP requests:

```bash
curl -X POST http://127.0.0.1:8080/webhooks/airtel \
  -H "Content-Type: application/json" \
  -d '{
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
  }'
```

### Health Check

```bash
curl http://127.0.0.1:8080/health
```

## TLS/HTTPS Support

The callback server supports TLS encryption for secure HTTPS communication in production environments.

### Certificate Requirements

- **Certificate File**: Valid X.509 certificate in PEM format
- **Private Key File**: Corresponding private key in PEM format (PKCS8 supported)
- **File Permissions**: Certificate and key files should have restrictive permissions (600 or 400)

### Certificate Generation

#### Self-Signed Certificate (Development Only)

```bash
# Generate private key and certificate
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Set appropriate permissions
chmod 600 key.pem
chmod 644 cert.pem
```

#### Production Certificate (Let's Encrypt)

```bash
# Install certbot
sudo apt-get install certbot

# Generate certificate
sudo certbot certonly --standalone -d your-domain.com

# Copy certificates to your application directory
sudo cp /etc/letsencrypt/live/your-domain.com/fullchain.pem cert.pem
sudo cp /etc/letsencrypt/live/your-domain.com/privkey.pem key.pem
```

### TLS Configuration Examples

#### Environment-based TLS

```bash
export TLS_CERT_PATH=cert.pem
export TLS_KEY_PATH=key.pem
export CALLBACK_PORT=443
```

```rust
// Will automatically use TLS if both cert and key paths are set
let config = CallbackServerConfig::default();
```

#### Programmatic TLS

```rust
use airtel_rs::CallbackServerConfig;

let config = CallbackServerConfig {
    host: "0.0.0.0".to_string(),
    port: 443,
    webhook_secret: Some("production_secret".to_string()),
    cert_path: Some("cert.pem".to_string()),
    key_path: Some("key.pem".to_string()),
};

// Check if TLS is enabled
if config.is_tls_enabled() {
    println!("🔒 TLS enabled for secure connections");
}
```

#### Builder Pattern TLS

```rust
let config = CallbackServerConfig::https_default()
    .with_tls("production_cert.pem".to_string(), "production_key.pem".to_string());
```

### TLS Testing

```bash
# Test HTTPS endpoint
curl -k https://127.0.0.1:8443/health

# Send test callback over HTTPS
curl -k -X POST https://127.0.0.1:8443/webhooks/airtel \
  -H "Content-Type: application/json" \
  -d '{
    "transaction": {
      "id": "tx_123",
      "airtel_money_id": "AM_456", 
      "status": "SUCCESS"
    },
    "amount": 1000,
    "currency": "KES"
  }'
```

### Certificate Validation

The server validates certificates on startup:

- Checks if certificate file exists and is readable
- Validates certificate format (PEM)
- Verifies private key exists and matches certificate
- Creates secure TLS configuration

If validation fails, the server will log detailed error messages and refuse to start.

## Production Deployment

### Security Considerations

1. **HTTPS**: Use HTTPS in production with proper TLS certificates
2. **Webhook Secret**: Always use webhook secrets for callback validation
3. **IP Filtering**: Restrict access to known Airtel Money IP addresses
4. **Rate Limiting**: Implement rate limiting to prevent abuse

### Monitoring

1. **Health Checks**: Use `/health` endpoint for load balancer health checks
2. **Logging**: Enable structured logging for callback processing
3. **Metrics**: Monitor callback volume and processing times
4. **Alerts**: Set up alerts for failed callback processing

### Example Production Setup

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Production configuration
    let config = CallbackServerConfig {
        host: "0.0.0.0".to_string(),
        port: 443, // HTTPS
        webhook_secret: std::env::var("AIRTEL_WEBHOOK_SECRET")
            .expect("AIRTEL_WEBHOOK_SECRET must be set"),
    };

    // Start server with error handling
    let callback_stream = start_callback_server(config).await
        .expect("Failed to start callback server");
    
    futures_util::pin_mut!(callback_stream);

    // Process callbacks with error handling
    while let Some(update) = callback_stream.next().await {
        if let Err(e) = process_callback_with_retry(&update).await {
            log::error!("Failed to process callback: {}", e);
            // Send to dead letter queue or retry mechanism
        }
    }

    Ok(())
}
```

## Similar Implementation Reference

This implementation is inspired by the MTN MoMo callback server pattern used in `/home/ondonda/rust/momo.rs/momo-callback-server/src/main.rs`, adapted for Airtel Money's callback structure and integrated as a feature within the main SDK.