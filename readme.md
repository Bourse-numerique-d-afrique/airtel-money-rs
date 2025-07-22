# Airtel Money Rust SDK

<div align="center">

[![Build Tests](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/airtel_rs.svg)](https://crates.io/crates/airtel_rs)
[![MIT licensed](https://img.shields.io/badge/License-MIT-yellow.svg)](https://choosealicense.com/licenses/mit/)
[![Docs](https://img.shields.io/badge/docs-yes-brightgreen.svg)](https://docs.rs/airtel_rs/)
[![Security Audit](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/security.yml/badge.svg)](https://github.com/Bourse-numerique-d-afrique/airtel_money_rs/actions/workflows/security.yml)

![Airtel Money logo](https://raw.githubusercontent.com/Bourse-numerique-d-afrique/airtel_money_rs/master/images/Airtel-Money.png)

</div>

A comprehensive Rust SDK for the Airtel Money API, supporting both Sandbox and Production environments.

## Features

✅ **Complete API Coverage**: All Airtel Money products supported:
- 💰 **Collections** - USSD Push payments, refunds, and status checking
- 📤 **Disbursements** - Money transfers to mobile wallets
- 🏦 **Account Management** - Balance inquiries
- 💵 **Cash In/Out** - Agent cash transactions
- 🌍 **Remittances** - International money transfers
- 🔄 **Callback Server** - Standalone webhook server for payment notifications

✅ **Multi-Country Support**: 14+ African countries supported
✅ **Type-Safe**: Fully typed request/response structures
✅ **Async/Await**: Modern async Rust with tokio
✅ **Automatic Token Management**: OAuth2 token refresh handling
✅ **Comprehensive Testing**: Unit and integration tests included
✅ **Production Ready**: Built with error handling and proper logging
✅ **Installable Binary**: Callback server can be installed with `cargo install`

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

The SDK includes a standalone callback server for handling Airtel Money webhook notifications:

### Installation

```bash
# Install the callback server binary
cargo install --path airtel_money_callback_server

# Or run directly
cd airtel_money_callback_server
cargo run
```

### Usage

```bash
# Set environment variables
export CALLBACK_PORT=8080
export CALLBACK_HOST=0.0.0.0
export AIRTEL_WEBHOOK_SECRET=your_webhook_secret

# Run the server
airtel_money_callback_server
```

### Available Endpoints

- `POST /webhooks/airtel` - Receives Airtel Money callbacks
- `GET /health` - Health check endpoint

The callback server automatically processes payment status updates and can be customized to integrate with your application's business logic.

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
cargo run --example basic_usage
```

## API Documentation

All API methods return properly typed responses that match the Airtel Money API specification. The SDK handles:

- ✅ Automatic token management and refresh
- ✅ Request/response serialization
- ✅ Error handling with descriptive messages
- ✅ Type-safe parameters and responses
- ✅ Environment switching (Sandbox/Production)

## Dependencies

- `reqwest` - HTTP client
- `serde` - JSON serialization
- `tokio` - Async runtime
- `chrono` - Date/time handling
- `uuid` - Unique ID generation

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
