/// Error handling module for the Airtel Money SDK
/// 
/// This module provides custom error types to replace the generic
/// `Box<dyn std::error::Error>` pattern and provide better error handling
/// with more specific error information.

use std::fmt;

/// Main error type for Airtel Money SDK operations
/// 
/// This enum covers all possible error scenarios that can occur
/// when using the Airtel Money API, providing better error handling
/// and debugging information.
/// 
/// # Examples
/// 
/// ```rust
/// use airtel_rs::AirtelError;
/// 
/// match some_api_call().await {
///     Ok(response) => println!("Success: {:?}", response),
///     Err(AirtelError::AuthenticationError { message }) => {
///         eprintln!("Authentication failed: {}", message);
///     }
///     Err(AirtelError::ApiError { status, message }) => {
///         eprintln!("API error {}: {}", status, message);
///     }
///     Err(e) => eprintln!("Other error: {}", e),
/// }
/// ```
#[derive(Debug)]
pub enum AirtelError {
    /// HTTP request/response errors from reqwest
    HttpError(reqwest::Error),
    
    /// API authentication/authorization errors
    AuthenticationError {
        /// Error message from the API
        message: String,
    },
    
    /// API-specific errors with status codes
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message from the API
        message: String,
    },
    
    /// JSON serialization/deserialization errors
    SerializationError(serde_json::Error),
    
    /// Token-related errors (expired, invalid, etc.)
    TokenError {
        /// Token error description
        message: String,
    },
    
    /// Network connectivity errors
    NetworkError {
        /// Network error description
        message: String,
    },
    
    /// Invalid configuration errors
    ConfigurationError {
        /// Configuration error description
        message: String,
    },
    
    /// Validation errors for request parameters
    ValidationError {
        /// Field that failed validation
        field: String,
        /// Validation error message
        message: String,
    },
}

impl fmt::Display for AirtelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AirtelError::HttpError(e) => write!(f, "HTTP error: {}", e),
            AirtelError::AuthenticationError { message } => {
                write!(f, "Authentication error: {}", message)
            }
            AirtelError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            AirtelError::SerializationError(e) => {
                write!(f, "Serialization error: {}", e)
            }
            AirtelError::TokenError { message } => {
                write!(f, "Token error: {}", message)
            }
            AirtelError::NetworkError { message } => {
                write!(f, "Network error: {}", message)
            }
            AirtelError::ConfigurationError { message } => {
                write!(f, "Configuration error: {}", message)
            }
            AirtelError::ValidationError { field, message } => {
                write!(f, "Validation error for '{}': {}", field, message)
            }
        }
    }
}

impl std::error::Error for AirtelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AirtelError::HttpError(e) => Some(e),
            AirtelError::SerializationError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for AirtelError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() || error.is_connect() {
            AirtelError::NetworkError {
                message: error.to_string(),
            }
        } else {
            AirtelError::HttpError(error)
        }
    }
}

impl From<serde_json::Error> for AirtelError {
    fn from(error: serde_json::Error) -> Self {
        AirtelError::SerializationError(error)
    }
}

/// Result type alias for Airtel Money SDK operations
/// 
/// This type alias provides a convenient way to return results
/// from SDK operations with the custom error type.
/// 
/// # Examples
/// 
/// ```rust
/// use airtel_rs::{AirtelResult, AirtelError};
/// 
/// async fn some_api_operation() -> AirtelResult<String> {
///     // API operation logic here
///     Ok("Success".to_string())
/// }
/// ```
pub type AirtelResult<T> = Result<T, AirtelError>;

/// Helper function to create an authentication error
/// 
/// # Arguments
/// 
/// * `message` - The authentication error message
/// 
/// # Examples
/// 
/// ```rust
/// use airtel_rs::errors::auth_error;
/// 
/// let error = auth_error("Invalid credentials");
/// ```
pub fn auth_error(message: &str) -> AirtelError {
    AirtelError::AuthenticationError {
        message: message.to_string(),
    }
}

/// Helper function to create an API error
/// 
/// # Arguments
/// 
/// * `status` - HTTP status code
/// * `message` - Error message
/// 
/// # Examples
/// 
/// ```rust
/// use airtel_rs::errors::api_error;
/// 
/// let error = api_error(400, "Bad Request");
/// ```
pub fn api_error(status: u16, message: &str) -> AirtelError {
    AirtelError::ApiError {
        status,
        message: message.to_string(),
    }
}

/// Helper function to create a validation error
/// 
/// # Arguments
/// 
/// * `field` - The field that failed validation
/// * `message` - Validation error message
/// 
/// # Examples
/// 
/// ```rust
/// use airtel_rs::errors::validation_error;
/// 
/// let error = validation_error("phone_number", "Invalid format");
/// ```
#[allow(dead_code)]
pub fn validation_error(field: &str, message: &str) -> AirtelError {
    AirtelError::ValidationError {
        field: field.to_string(),
        message: message.to_string(),
    }
}