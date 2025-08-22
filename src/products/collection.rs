//! Collection product module for merchant payment operations
//!
//! This module provides the Collection service which allows merchants to:
//! - Request payments from customers via USSD push notifications
//! - Check the status of payment transactions
//! - Process refunds for completed transactions
//!
//! # Examples
//!
//! ```rust
//! use airtel_rs::{AirtelMoney, Environment, Country};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
//!     let collection = airtel.collection(
//!         "client_id".to_string(),
//!         "client_secret".to_string()
//!     );
//!
//!     // Request payment from customer
//!     let payment = collection.ussd_push(
//!         "order_123".to_string(),        // Reference
//!         "254700000000".to_string(),     // Customer phone
//!         5000,                           // Amount (50.00 KES)
//!         "tx_456".to_string(),           // Transaction ID
//!     ).await?;
//!
//!     // Check payment status
//!     let status = collection.status(payment.transaction.id).await?;
//!
//!     // Refund if needed
//!     if status.transaction.status == "SUCCESS" {
//!         let refund = collection.refund(
//!             payment.transaction.airtel_money_id
//!         ).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```

use crate::authorization::get_valid_access_token;
use crate::requests::collection_refund_request::RefundCollectionRequest;
use crate::requests::ussd_push_request::{
    USSDSubscriberRequest, USSDTransactionRequest, UssdPushRequest,
};
use crate::responses::collection_refund_response::CollectionRefundResponse;
use crate::responses::collection_status_response::CollectionStatusResponse;
use crate::responses::collection_ussd_response::CollectionUSSDResponse;
use crate::{Country, Currency, Environment};

/// Collection client for merchant payment operations
///
/// The Collection service enables merchants to request payments from customers,
/// monitor payment status, and process refunds. This is the primary interface
/// for businesses that want to collect payments from Airtel Money users.
///
/// # Payment Flow
///
/// 1. **Request Payment**: Use `ussd_push()` to send a payment request to the customer
/// 2. **Customer Authorization**: Customer receives USSD prompt on their phone to authorize payment
/// 3. **Status Monitoring**: Use `status()` to check if payment was completed
/// 4. **Refund Processing**: Use `refund()` if a refund is needed
///
/// # Examples
///
/// ```rust
/// use airtel_rs::{Collection, Country, Currency, Environment};
///
/// let collection = Collection::new(
///     Country::Kenya,
///     Currency::KES,
///     Environment::Sandbox,
///     "your_client_id".to_string(),
///     "your_client_secret".to_string(),
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Collection {
    /// Target country for operations
    pub country: Country,
    /// Currency for transactions
    pub currency: Currency,
    /// API environment (Sandbox or Production)
    pub environment: Environment,
    /// OAuth2 client ID
    pub client_id: String,
    /// OAuth2 client secret
    pub client_secret: String,
}

impl Collection {
    /// Creates a new Collection client instance
    ///
    /// # Arguments
    ///
    /// * `country` - Target country for operations
    /// * `currency` - Currency for transactions
    /// * `environment` - API environment (Sandbox or Production)
    /// * `client_id` - OAuth2 client ID for authentication
    /// * `client_secret` - OAuth2 client secret for authentication
    ///
    /// # Returns
    ///
    /// A new Collection instance ready for payment operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{Collection, Country, Currency, Environment};
    ///
    /// let collection = Collection::new(
    ///     Country::Kenya,
    ///     Currency::KES,
    ///     Environment::Sandbox,
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string(),
    /// );
    /// ```
    pub fn new(
        country: Country,
        currency: Currency,
        environment: Environment,
        client_id: String,
        client_secret: String,
    ) -> Collection {
        Collection {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /// Initiates a USSD push payment request to a customer
    ///
    /// This method sends a payment request to the customer's mobile phone via USSD.
    /// The customer will receive a prompt on their phone to authorize the payment
    /// using their Airtel Money PIN.
    ///
    /// # Arguments
    ///
    /// * `reference` - Your internal reference for this payment (e.g., order ID)
    /// * `msisdn` - Customer's mobile number in international format (e.g., "254700000000")
    /// * `amount` - Payment amount in the smallest currency unit (e.g., cents for KES)
    /// * `id` - Unique transaction identifier (must be unique per request)
    ///
    /// # Returns
    ///
    /// Returns a `CollectionUSSDResponse` containing transaction details and status,
    /// or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    ///     let collection = airtel.collection(
    ///         "client_id".to_string(),
    ///         "client_secret".to_string()
    ///     );
    ///
    ///     let response = collection.ussd_push(
    ///         "order_12345".to_string(),      // Your order reference
    ///         "254700123456".to_string(),     // Customer phone number
    ///         5000,                           // 50.00 KES (amount in cents)
    ///         "tx_67890".to_string(),         // Unique transaction ID
    ///     ).await?;
    ///
    ///     println!("Transaction ID: {}", response.transaction.id);
    ///     println!("Status: {}", response.status.message);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - The customer must have sufficient balance in their Airtel Money wallet
    /// - The transaction will be pending until the customer authorizes it
    /// - Use the `status()` method to check if the payment was completed
    /// - Transaction IDs must be unique; duplicate IDs will be rejected
    pub async fn ussd_push(
        &self,
        reference: String,
        msisdn: String,
        amount: i32,
        id: String,
    ) -> Result<CollectionUSSDResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .post(format!("{}/merchant/v1/payments/", self.environment))
            .bearer_auth(access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string())
            .body(UssdPushRequest {
                reference,
                subscriber: USSDSubscriberRequest {
                    country: self.country,
                    msisdn,
                    currency: self.currency,
                },
                transaction: USSDTransactionRequest {
                    amount,
                    country: self.country,
                    currency: self.currency,
                    id,
                },
            });

        let res = req.send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let response: CollectionUSSDResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }

    /// Processes a refund for a completed payment transaction
    ///
    /// This method initiates a refund for a previously successful payment.
    /// The refund amount will be the full original transaction amount.
    ///
    /// # Arguments
    ///
    /// * `airtel_money_id` - The Airtel Money transaction ID from the original payment
    ///   (found in the `airtel_money_id` field of the payment response)
    ///
    /// # Returns
    ///
    /// Returns a `CollectionRefundResponse` containing refund transaction details,
    /// or an error if the refund fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    ///     let collection = airtel.collection(
    ///         "client_id".to_string(),
    ///         "client_secret".to_string()
    ///     );
    ///
    ///     // First, make a payment
    ///     let payment = collection.ussd_push(
    ///         "order_123".to_string(),
    ///         "254700000000".to_string(),
    ///         1000,
    ///         "tx_456".to_string(),
    ///     ).await?;
    ///
    ///     // Later, process a refund using the Airtel Money ID
    ///     let refund = collection.refund(
    ///         payment.transaction.airtel_money_id
    ///     ).await?;
    ///
    ///     println!("Refund processed: {}", refund.transaction.id);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - Only successful transactions can be refunded
    /// - Refunds are processed for the full original amount
    /// - The original transaction must exist and be in a successful state
    /// - Partial refunds are not supported through this method
    pub async fn refund(
        &self,
        airtel_money_id: String,
    ) -> Result<CollectionRefundResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let req = client
            .post(format!("{}/standard/v1/payments/refund", self.environment))
            .bearer_auth(access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string())
            .body(RefundCollectionRequest { airtel_money_id });

        let res = req.send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let response: CollectionRefundResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }

    /// Retrieves the current status of a payment transaction
    ///
    /// This method allows you to check the current status of a payment transaction
    /// using the transaction ID. This is useful for monitoring whether a USSD push
    /// payment has been completed by the customer.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transaction ID from the original payment request
    ///   (the `id` field from the USSD push response)
    ///
    /// # Returns
    ///
    /// Returns a `CollectionStatusResponse` containing the current transaction status
    /// and details, or an error if the status check fails.
    ///
    /// # Transaction Statuses
    ///
    /// - `"PENDING"` - Customer has not yet authorized the payment
    /// - `"SUCCESS"` - Payment completed successfully
    /// - `"FAILED"` - Payment failed or was declined
    /// - `"TIMEOUT"` - Customer did not respond within the time limit
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    ///     let collection = airtel.collection(
    ///         "client_id".to_string(),
    ///         "client_secret".to_string()
    ///     );
    ///
    ///     // Make a payment request
    ///     let payment = collection.ussd_push(
    ///         "order_123".to_string(),
    ///         "254700000000".to_string(),
    ///         1000,
    ///         "tx_456".to_string(),
    ///     ).await?;
    ///
    ///     // Check the payment status
    ///     let status = collection.status(payment.transaction.id).await?;
    ///
    ///     match status.transaction.status.as_str() {
    ///         "SUCCESS" => println!("Payment completed successfully"),
    ///         "PENDING" => println!("Waiting for customer authorization"),
    ///         "FAILED" => println!("Payment failed"),
    ///         _ => println!("Unknown status: {}", status.transaction.status),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - Status checks can be performed multiple times for the same transaction
    /// - It's recommended to implement polling with reasonable intervals
    /// - Some transactions may take a few minutes to complete
    pub async fn status(
        &self,
        transaction_id: String,
    ) -> Result<CollectionStatusResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let access_token =
            get_valid_access_token(self.environment, &self.client_id, &self.client_secret).await?;
        let res = client
            .get(format!(
                "{}/standard/v1/payments/status/{}",
                self.environment, transaction_id
            ))
            .bearer_auth(access_token.access_token)
            .header("Content-Type", "application/json")
            .header("X-Country", self.country.to_string())
            .header("X-Currency", self.currency.to_string())
            .send()
            .await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let response: CollectionStatusResponse = serde_json::from_str(&body)?;
            Ok(response)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }
}
