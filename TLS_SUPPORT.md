# TLS Support Added to Airtel Money Callback Server

## Summary

TLS/HTTPS support has been successfully added to the Airtel Money callback server, providing secure encrypted communication for production deployments.

## What Was Added

### 🔧 Dependencies
- `tokio-rustls` v0.24 - Async TLS implementation
- `rustls` v0.21 - Pure Rust TLS library
- `rustls-pemfile` v1.0 - PEM file parsing
- `warp` with TLS features enabled

### 📋 Configuration Updates
- `cert_path: Option<String>` - Path to TLS certificate file
- `key_path: Option<String>` - Path to TLS private key file
- `is_tls_enabled()` method to check TLS status
- `with_tls()` builder method for easy TLS configuration
- `https_default()` method for HTTPS-ready defaults

### 🔒 TLS Implementation
- Certificate validation on server startup
- Automatic HTTP vs HTTPS server selection based on config
- PEM file loading and parsing
- TLS configuration creation with secure defaults
- Comprehensive error handling with detailed messages

### 📚 Documentation & Examples
- Updated `CALLBACK_SERVER.md` with TLS section
- New `callback_server_tls.rs` example
- Certificate generation instructions
- Production deployment guidelines
- Security best practices

### 🧪 Tests
- TLS configuration validation tests
- Builder pattern tests
- HTTPS defaults tests
- Integration tests for server startup

## Usage Examples

### Environment Variables
```bash
export TLS_CERT_PATH=cert.pem
export TLS_KEY_PATH=key.pem
export CALLBACK_PORT=443
```

### Programmatic Configuration
```rust
// HTTPS with explicit paths
let config = CallbackServerConfig {
    host: "0.0.0.0".to_string(),
    port: 443,
    webhook_secret: Some("secret".to_string()),
    cert_path: Some("cert.pem".to_string()),
    key_path: Some("key.pem".to_string()),
};

// Builder pattern
let config = CallbackServerConfig::default()
    .with_tls("cert.pem".to_string(), "key.pem".to_string());

// HTTPS defaults
let config = CallbackServerConfig::https_default();
```

### Server Behavior
- **TLS Enabled**: Starts HTTPS server on specified port (443 default)
- **TLS Disabled**: Starts HTTP server with security warning
- **Certificate Validation**: Validates certificates before starting
- **Graceful Fallback**: Clear error messages if TLS setup fails

## Security Features

### Certificate Validation
- Checks file existence and readability
- Validates PEM format
- Verifies certificate-key pair matching
- Secure TLS configuration with safe defaults

### Production Ready
- No client authentication required (suitable for webhooks)
- Support for certificate chains
- Proper error handling and logging
- Compatible with Let's Encrypt and other CA certificates

### Development Support
- Self-signed certificate instructions
- Development vs production configuration examples
- Testing commands for HTTPS endpoints

## Backward Compatibility

- All existing HTTP configurations continue to work
- TLS is completely optional
- No breaking changes to existing API
- Feature-gated to avoid unnecessary dependencies

## Testing

All tests pass including:
- 10 callback server tests (including new TLS tests)
- Example compilation verification
- Build testing with and without features
- Integration test compatibility

The implementation follows the same pattern as the MTN MoMo callback server reference but with additional TLS security features specifically designed for the Airtel Money API.