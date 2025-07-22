/// HTTP client module for Airtel Money API operations
/// 
/// This module provides a centralized HTTP client with common functionality
/// to eliminate code duplication across product modules and provide
/// consistent request/response handling.

use crate::{
    config::ProductConfig,
    errors::{AirtelResult, api_error, auth_error},
    authorization::get_valid_access_token,
};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

/// Centralized HTTP client for Airtel Money API operations
/// 
/// This client provides common functionality for all API requests,
/// including authentication, header management, and response handling.
/// It eliminates code duplication across product modules.
/// 
/// # Examples
/// 
/// ```rust
/// use airtel_rs::{ApiClient, ProductConfig, Environment, Country, Currency};
/// 
/// let config = ProductConfig::new(
///     Environment::Sandbox,
///     Country::Kenya,
///     Currency::KES,
///     "client_id".to_string(),
///     "client_secret".to_string(),
/// );
/// 
/// let client = ApiClient::new(config);
/// ```
#[derive(Debug)]
pub struct ApiClient {
    /// Product configuration containing credentials and settings
    config: ProductConfig,
    /// Underlying HTTP client
    client: Client,
}

impl ApiClient {
    /// Creates a new ApiClient instance
    /// 
    /// # Arguments
    /// 
    /// * `config` - Product configuration with credentials and settings
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use airtel_rs::{ApiClient, ProductConfig, Environment, Country, Currency};
    /// 
    /// let config = ProductConfig::new(
    ///     Environment::Sandbox,
    ///     Country::Kenya,
    ///     Currency::KES,
    ///     "client_id".to_string(),
    ///     "client_secret".to_string(),
    /// );
    /// 
    /// let client = ApiClient::new(config);
    /// ```
    pub fn new(config: ProductConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Makes an authenticated GET request to the API
    /// 
    /// This method handles token retrieval, authentication headers,
    /// and common headers automatically.
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - API endpoint path (e.g., "/standard/v1/users/balance")
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing the parsed response or an AirtelError
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use airtel_rs::{ApiClient, ProductConfig, Environment, Country, Currency};
    /// use serde::{Deserialize, Serialize};
    /// 
    /// #[derive(Deserialize)]
    /// struct BalanceResponse {
    ///     data: BalanceData,
    /// }
    /// 
    /// #[derive(Deserialize)]
    /// struct BalanceData {
    ///     balance: String,
    /// }
    /// 
    /// async fn get_balance(client: &ApiClient) -> Result<BalanceResponse, AirtelError> {
    ///     client.get("/standard/v1/users/balance").await
    /// }
    /// ```
    pub async fn get<T>(&self, endpoint: &str) -> AirtelResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let access_token = get_valid_access_token(
            self.config.environment,
            &self.config.client_id,
            &self.config.client_secret,
        )
        .await
        .map_err(|e| auth_error(&e.to_string()))?;

        let url = format!("{}{}", self.config.base_url(), endpoint);
        
        let response = self
            .client
            .get(&url)
            .bearer_auth(&access_token.access_token)
            .header("Accept", "*/*")
            .header("X-Country", self.config.country_code())
            .header("X-Currency", self.config.currency_code())
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Makes an authenticated POST request to the API
    /// 
    /// This method handles token retrieval, authentication headers,
    /// common headers, and JSON serialization automatically.
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - API endpoint path
    /// * `body` - Request body that implements Serialize
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing the parsed response or an AirtelError
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use airtel_rs::{ApiClient, ProductConfig, Environment, Country, Currency};
    /// use serde::{Deserialize, Serialize};
    /// 
    /// #[derive(Serialize)]
    /// struct PaymentRequest {
    ///     amount: i32,
    ///     msisdn: String,
    /// }
    /// 
    /// #[derive(Deserialize)]
    /// struct PaymentResponse {
    ///     transaction_id: String,
    /// }
    /// 
    /// async fn make_payment(
    ///     client: &ApiClient,
    ///     request: PaymentRequest
    /// ) -> Result<PaymentResponse, AirtelError> {
    ///     client.post("/merchant/v1/payments/", request).await
    /// }
    /// ```
    pub async fn post<T, B>(&self, endpoint: &str, body: B) -> AirtelResult<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let access_token = get_valid_access_token(
            self.config.environment,
            &self.config.client_id,
            &self.config.client_secret,
        )
        .await
        .map_err(|e| auth_error(&e.to_string()))?;

        let url = format!("{}{}", self.config.base_url(), endpoint);
        
        let response = self
            .client
            .post(&url)
            .bearer_auth(&access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.config.country_code())
            .header("X-Currency", self.config.currency_code())
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Makes an authenticated POST request with custom headers
    /// 
    /// This method allows passing additional headers while still
    /// handling authentication and common headers automatically.
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - API endpoint path
    /// * `body` - Request body that implements Serialize
    /// * `additional_headers` - Vector of (key, value) header pairs
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing the parsed response or an AirtelError
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use airtel_rs::{ApiClient, ProductConfig, Environment, Country, Currency};
    /// use serde::{Deserialize, Serialize};
    /// 
    /// #[derive(Serialize)]
    /// struct SecureRequest {
    ///     amount: i32,
    /// }
    /// 
    /// async fn secure_operation(client: &ApiClient) -> Result<(), AirtelError> {
    ///     let headers = vec![
    ///         ("x-signature".to_string(), "signature_value".to_string()),
    ///         ("x-key".to_string(), "api_key_value".to_string()),
    ///     ];
    ///     
    ///     let request = SecureRequest { amount: 1000 };
    ///     let _response: serde_json::Value = client
    ///         .post_with_headers("/secure/endpoint", request, headers)
    ///         .await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn post_with_headers<T, B>(
        &self,
        endpoint: &str,
        body: B,
        additional_headers: Vec<(String, String)>,
    ) -> AirtelResult<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let access_token = get_valid_access_token(
            self.config.environment,
            &self.config.client_id,
            &self.config.client_secret,
        )
        .await
        .map_err(|e| auth_error(&e.to_string()))?;

        let url = format!("{}{}", self.config.base_url(), endpoint);
        
        let mut request_builder = self
            .client
            .post(&url)
            .bearer_auth(&access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.config.country_code())
            .header("X-Currency", self.config.currency_code())
            .json(&body);

        // Add additional headers
        for (key, value) in additional_headers {
            request_builder = request_builder.header(key, value);
        }

        let response = request_builder.send().await?;
        self.handle_response(response).await
    }

    /// Handles HTTP response and converts to typed result
    /// 
    /// This private method provides consistent response handling
    /// across all HTTP methods, including error conversion and
    /// JSON deserialization.
    /// 
    /// # Arguments
    /// 
    /// * `response` - HTTP response from reqwest
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing the parsed response or an AirtelError
    async fn handle_response<T>(&self, response: Response) -> AirtelResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();
        let status_code = status.as_u16();

        if status.is_success() {
            let body = response.text().await?;
            let parsed_response: T = serde_json::from_str(&body)?;
            Ok(parsed_response)
        } else {
            let error_body = response.text().await.unwrap_or_default();
            
            // Try to parse as JSON error response
            if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&error_body) {
                let message = error_json
                    .get("message")
                    .or_else(|| error_json.get("error_description"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&error_body);
                
                if status_code == 401 || message.to_lowercase().contains("unauthorized") {
                    Err(auth_error(message))
                } else {
                    Err(api_error(status_code, message))
                }
            } else {
                // Non-JSON error response
                if status_code == 401 {
                    Err(auth_error(&error_body))
                } else {
                    Err(api_error(status_code, &error_body))
                }
            }
        }
    }

    /// Returns a reference to the client configuration
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use airtel_rs::{ApiClient, ProductConfig, Environment, Country, Currency};
    /// 
    /// let config = ProductConfig::new(
    ///     Environment::Sandbox,
    ///     Country::Kenya,
    ///     Currency::KES,
    ///     "client_id".to_string(),
    ///     "client_secret".to_string(),
    /// );
    /// 
    /// let client = ApiClient::new(config);
    /// let config_ref = client.config();
    /// 
    /// assert_eq!(config_ref.country, Country::Kenya);
    /// ```
    pub fn config(&self) -> &ProductConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Environment, Country, Currency};

    #[test]
    fn test_api_client_creation() {
        let config = ProductConfig::new(
            Environment::Sandbox,
            Country::Kenya,
            Currency::KES,
            "test_id".to_string(),
            "test_secret".to_string(),
        );

        let client = ApiClient::new(config);
        assert_eq!(client.config().country, Country::Kenya);
        assert_eq!(client.config().currency, Currency::KES);
        assert_eq!(client.config().environment, Environment::Sandbox);
    }

    #[test]
    fn test_base_url_generation() {
        let sandbox_config = ProductConfig::new(
            Environment::Sandbox,
            Country::Kenya,
            Currency::KES,
            "test_id".to_string(),
            "test_secret".to_string(),
        );

        let production_config = ProductConfig::new(
            Environment::Production,
            Country::Kenya,
            Currency::KES,
            "test_id".to_string(),
            "test_secret".to_string(),
        );

        assert_eq!(sandbox_config.base_url(), "https://openapiuat.airtel.africa");
        assert_eq!(production_config.base_url(), "https://openapi.airtel.africa");
    }
}