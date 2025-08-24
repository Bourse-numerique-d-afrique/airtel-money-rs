# Airtel Money Rust SDK

<div align="center">

[![Build Tests](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/airtel_rs.svg)](https://crates.io/crates/airtel_rs)
[![MIT licensed](https://img.shields.io/badge/License-MIT-yellow.svg)](https://choosealicense.com/licenses/mit/)
[![Docs](https://img.shields.io/badge/docs-yes-brightgreen.svg)](https://docs.rs/airtel_rs/)
[![Security Audit](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/security.yml/badge.svg)](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/security.yml)

![Airtel Money logo](https://raw.githubusercontent.com/Bourse-numerique-d-afrique/airtel_money_rs/master/images/airtel_momo.png)

</div>

A comprehensive Rust SDK for the Airtel Money API, supporting both Sandbox and Production environments.

## Features

✅ **Complete API Coverage**: All Airtel Money products supported:
- 💰 **Collections** - USSD Push payments, refunds, and status checking
- 📤 **Disbursements** - Money transfers to mobile wallets
- 🏦 **Account Management** - Balance inquiries
- 💵 **Cash In/Out** - Agent cash transactions
- 🌍 **Remittances** - International money transfers
- 🔄 **Callback Server** - Integrated webhook server with TLS support for payment notifications

✅ **Multi-Country Support**: 14+ African countries supported
✅ **Type-Safe**: Fully typed request/response structures
✅ **Async/Await**: Modern async Rust with tokio
✅ **Automatic Token Management**: OAuth2 token refresh handling
✅ **Stream-based Callbacks**: Process webhooks as async streams
✅ **TLS/HTTPS Support**: Production-ready secure callback server
✅ **Comprehensive Testing**: Unit and integration tests included
✅ **Production Ready**: Built with error handling and proper logging

## Supported Countries & Currencies

| Country | Currency | Code |
|---------|----------|------|
| Kenya | KES | KE |
| Uganda | UGX | UG |
| Tanzania | TZS | TZ |
| Nigeria | NGN | NG |
| Rwanda | RWF | RW |
| Malawi | MWK | MW |
| Zambia | ZMW | ZM |
| Madagascar | MGA | MG |
| DRC | CDF | CD |
| Gabon | XAF | GA |
| Chad | XAF | TD |
| Niger | XOF | NE |
| Congo B | XAF | CG |
| Seychelles | SCR | SC |

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
airtel_rs = { path = "." }
tokio = { version = "1.0", features = ["full"] }
```

## Basic Usage

```rust
use airtel_rs::{AirtelMoney, Environment, Country};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    
    // Set up credentials
    let client_id = "your_client_id".to_string();
    let client_secret = "your_client_secret".to_string();

    // Get account balance
    let account = airtel.account(client_id.clone(), client_secret.clone());
    let balance = account.get_balance().await?;
    println!("Balance: {}", balance.data.balance);

    // Initiate a collection
    let collection = airtel.collection(client_id.clone(), client_secret.clone());
    let result = collection.ussd_push(
        "reference123".to_string(),    // Reference
        "254700000000".to_string(),    // Phone number
        1000,                          // Amount
        "tx123".to_string()            // Transaction ID
    ).await?;
    
    Ok(())
}
```

## API Examples

### Collections (USSD Push)
```rust
let collection = airtel.collection(client_id, client_secret);
let result = collection.ussd_push(reference, msisdn, amount, tx_id).await?;
let status = collection.status(transaction_id).await?;
let refund = collection.refund(airtel_money_id).await?;
```

### Disbursements
```rust
let disbursement = airtel.disbursement(client_id, client_secret);
let result = disbursement.disburse(msisdn, amount, tx_id, reference, pin).await?;
let status = disbursement.get_status(transaction_id).await?;
```

### Cash In/Out
```rust
let cash_in = airtel.cash_in(client_id, client_secret);
let result = cash_in.cash_in(msisdn, amount, tx_id, reference, pin, remark).await?;

let cash_out = airtel.cash_out(client_id, client_secret);
let result = cash_out.cash_out(msisdn, amount, tx_id, reference, pin, remark).await?;
```

### Remittances
```rust
let remittance = airtel.remittance(client_id, client_secret);

// Check eligibility
let eligibility = remittance.check_eligibility(msisdn, amount, country, currency).await?;

// Transfer money
let transfer = remittance.money_transfer_credit(
    amount, ext_id, msisdn, payer_country, 
    first_name, last_name, pin
).await?;

// Check status
let status = remittance.money_transfer_status(ext_tr_id).await?;
```

## Callback Server

The SDK includes an integrated callback server feature for handling Airtel Money webhook notifications as a stream of events:

### Enable the Feature

```toml
[dependencies]
airtel_rs = { version = "*", features = ["callback_server"] }
futures-util = "0.3"
```

### Basic HTTP Server

```rust
use airtel_rs::callback_server::{start_callback_server, CallbackServerConfig};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the callback server
    let config = CallbackServerConfig {
        host: "0.0.0.0".to_string(),
        port: 8080,
        webhook_secret: Some("your_webhook_secret".to_string()),
        cert_path: None,  // HTTP for development
        key_path: None,
    };

    // Start the server and get a stream of callbacks
    let callback_stream = start_callback_server(config).await?;
    futures_util::pin_mut!(callback_stream);

    println!("🚀 Callback server started on http://0.0.0.0:8080");
    println!("📡 Endpoints: POST /webhooks/airtel, GET /health");

    // Process callbacks as they arrive
    while let Some(update) = callback_stream.next().await {
        println!("📥 Received callback: {:?}", update.callback_type);
        
        match update.response.transaction.status.as_str() {
            "SUCCESS" => {
                println!("✅ Payment successful: {} {}", 
                    update.response.amount, update.response.currency);
                // Update database, send notifications, etc.
                process_successful_payment(&update).await;
            }
            "FAILED" => {
                println!("❌ Payment failed: {}", 
                    update.response.transaction.id);
                // Handle failure, notify customer, etc.
                process_failed_payment(&update).await;
            }
            "PENDING" => {
                println!("🔄 Payment pending: {}", 
                    update.response.transaction.id);
                // Monitor payment status
            }
            _ => {
                println!("❓ Unknown status: {}", 
                    update.response.transaction.status);
            }
        }
    }

    Ok(())
}

async fn process_successful_payment(update: &airtel_rs::AirtelUpdates) {
    // Your business logic here
    println!("Processing successful payment for transaction: {}", 
        update.response.transaction.id);
    
    // Example: Update database
    // database::update_payment_status(&update.response.transaction.id, "completed").await;
    
    // Example: Send confirmation email
    // email::send_payment_confirmation(&update.response.reference).await;
    
    // Example: Fulfill order
    // orders::process_order(&update.response.transaction.id).await;
}

async fn process_failed_payment(update: &airtel_rs::AirtelUpdates) {
    // Your failure handling logic here
    println!("Processing failed payment for transaction: {}", 
        update.response.transaction.id);
    
    // Example: Update database
    // database::update_payment_status(&update.response.transaction.id, "failed").await;
    
    // Example: Notify customer
    // notifications::send_failure_notification(&update.response.reference).await;
}
```

### HTTPS/TLS Server (Production)

```rust
use airtel_rs::callback_server::{start_callback_server, CallbackServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // HTTPS configuration for production
    let config = CallbackServerConfig {
        host: "0.0.0.0".to_string(),
        port: 443,
        webhook_secret: Some("production_webhook_secret".to_string()),
        cert_path: Some("/path/to/cert.pem".to_string()),
        key_path: Some("/path/to/key.pem".to_string()),
    };

    // Alternative: Use builder pattern
    // let config = CallbackServerConfig::https_default()
    //     .with_tls("cert.pem".to_string(), "key.pem".to_string());

    let callback_stream = start_callback_server(config).await?;
    futures_util::pin_mut!(callback_stream);

    println!("🔒 Secure HTTPS callback server started on port 443");
    
    // Process callbacks with enhanced security
    while let Some(update) = callback_stream.next().await {
        // All communications are now encrypted with TLS
        process_secure_callback(update).await;
    }

    Ok(())
}
```

### Environment Configuration

```bash
# Basic HTTP setup
export CALLBACK_HOST=0.0.0.0
export CALLBACK_PORT=8080
export AIRTEL_WEBHOOK_SECRET=your_webhook_secret

# HTTPS/TLS setup (production)
export TLS_CERT_PATH=/path/to/cert.pem
export TLS_KEY_PATH=/path/to/key.pem
export CALLBACK_PORT=443
```

Then use:
```rust
// Automatically uses environment variables
let config = CallbackServerConfig::default();
let callback_stream = start_callback_server(config).await?;
```

### Available Endpoints

- `POST /webhooks/airtel` - Receives Airtel Money callbacks
- `GET /health` - Health check endpoint (returns JSON with status and version)

### Key Features

- **🔄 Stream-based**: Process callbacks as async stream of events
- **🔒 TLS Support**: Production-ready HTTPS with certificate validation
- **⚙️ Flexible Config**: Environment variables, builder patterns, or direct configuration  
- **🛡️ Type Safety**: Strongly-typed callback structures
- **📊 Health Monitoring**: Built-in health check endpoint
- **🔧 Production Ready**: Comprehensive error handling and logging

For more examples, see [`examples/callback_server_usage.rs`](examples/callback_server_usage.rs) and [`examples/callback_server_tls.rs`](examples/callback_server_tls.rs).

## Environment Setup

Set your credentials as environment variables:

```bash
export AIRTEL_CLIENT_ID=your_client_id
export AIRTEL_CLIENT_SECRET=your_client_secret
```

## Testing

The SDK includes comprehensive test suites:

### Unit Tests (No credentials required)
```bash
# Run basic unit tests
cargo test

# Run callback server tests
cargo test callback_server --features callback_server

# Run SDK functionality demo tests
cargo test --test mock_demo_tests -- --nocapture
```

### Integration Tests (Requires valid credentials)
```bash
# Run diagnostic test to check credentials
cargo test --test diagnostic_test -- --nocapture

# Run comprehensive integration tests
cargo test --test live_api_tests -- --nocapture

# Run specific integration test
cargo test --test live_api_tests test_account_balance -- --nocapture

# Use the interactive test runner
./run_integration_tests.sh
```

### Setting up Credentials for Integration Tests

1. **Get Airtel Money API credentials** from Airtel Money Developer Portal
2. **Create a `.env` file** in the project root:
```bash
AIRTEL_CLIENT_ID=your_client_id_here
AIRTEL_CLIENT_SECRET=your_client_secret_here
AIRTEL_PIN=your_pin_here
```
3. **Run the diagnostic test** to verify credentials:
```bash
cargo test --test diagnostic_test -- --nocapture
```

### Available Integration Tests

- ✅ **test_token_generation_and_refresh** - OAuth2 token management
- ✅ **test_account_balance** - Account balance retrieval
- ✅ **test_collection_ussd_push** - USSD push payments
- ✅ **test_disbursement** - Money disbursements
- ✅ **test_cash_in** - Cash in transactions
- ✅ **test_cash_out** - Cash out transactions
- ✅ **test_remittance_eligibility** - Remittance eligibility checks
- ✅ **test_remittance_transfer_credit** - Remittance transfers
- ✅ **test_collection_refund** - Payment refunds
- ✅ **test_multiple_countries** - Multi-country support
- ✅ **test_error_handling** - Error handling validation
- ✅ **test_concurrent_requests** - Concurrent API calls

## Examples

Check the `examples/` directory for more detailed usage:

```bash
# Basic SDK usage
cargo run --example basic_usage

# Callback server examples
cargo run --example callback_server_usage --features callback_server
cargo run --example callback_server_tls --features callback_server
```

## API Documentation

All API methods return properly typed responses that match the Airtel Money API specification. The SDK handles:

- ✅ Automatic token management and refresh
- ✅ Request/response serialization
- ✅ Error handling with descriptive messages
- ✅ Type-safe parameters and responses
- ✅ Environment switching (Sandbox/Production)

## Dependencies

### Core Dependencies
- `reqwest` - HTTP client
- `serde` - JSON serialization
- `tokio` - Async runtime
- `chrono` - Date/time handling
- `uuid` - Unique ID generation

### Callback Server Dependencies (optional, with `callback_server` feature)
- `warp` - Web framework for HTTP server
- `tokio-rustls` - TLS/HTTPS support
- `rustls` - Pure Rust TLS implementation
- `async-stream` - Async stream utilities
- `env_logger` - Logging support

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create your feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Support

For issues and feature requests, please create an issue on GitHub.
