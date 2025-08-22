//! Airtel Money Rust SDK
//!
//! A comprehensive Rust SDK for integrating with the Airtel Money API.
//! This SDK provides easy-to-use interfaces for all Airtel Money services
//! including Collections, Disbursements, Remittances, Cash In/Out, and Account operations.
//!
//! # Features
//!
//! - **Multi-country support**: Supports 14 African countries
//! - **Async/await**: Built with modern async Rust patterns
//! - **Type safety**: Strongly typed request/response structures
//! - **Environment support**: Both Sandbox and Production environments
//! - **Token management**: Automatic OAuth2 token handling with refresh
//! - **Error handling**: Comprehensive error types with detailed information
//!
//! # Supported Countries and Currencies
//!
//! | Country | Currency |
//! |---------|----------|
//! | Kenya (KE) | KES |
//! | Uganda (UG) | UGX |
//! | Tanzania (TZ) | TZS |
//! | Madagascar (MG) | MGA |
//! | Democratic Republic of Congo (CD) | CDF |
//! | Zambia (ZM) | ZMW |
//! | Seychelles (SC) | SCR |
//! | Rwanda (RW) | RWF |
//! | Malawi (MW) | MWK |
//! | Nigeria (NG) | NGN |
//! | Niger (NE) | XOF |
//! | Chad (TD) | XAF |
//! | Gabon (GA) | XAF |
//! | Republic of Congo (CG) | XAF |
//!
//! # Quick Start
//!
//! ```rust
//! use airtel_rs::{AirtelMoney, Environment, Country};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the SDK
//!     let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
//!
//!     // Create a collection instance
//!     let collection = airtel.collection(
//!         "your_client_id".to_string(),
//!         "your_client_secret".to_string()
//!     );
//!
//!     // Make a USSD push collection request
//!     let response = collection.ussd_push(
//!         "reference_123".to_string(),
//!         "254700000000".to_string(),  // Phone number
//!         1000,                         // Amount in smallest currency unit
//!         "transaction_123".to_string(),
//!     ).await?;
//!
//!     println!("Transaction ID: {}", response.transaction.id);
//!     Ok(())
//! }
//! ```
//!
//! # Examples
//!
//! ## Collections (Merchant Payments)
//!
//! ```rust
//! use airtel_rs::{AirtelMoney, Environment, Country};
//!
//! async fn collect_payment() -> Result<(), Box<dyn std::error::Error>> {
//!     let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
//!     let collection = airtel.collection("client_id".to_string(), "secret".to_string());
//!
//!     // USSD Push - prompt customer to pay
//!     let payment = collection.ussd_push(
//!         "order_12345".to_string(),      // Your reference
//!         "254700000000".to_string(),     // Customer phone
//!         5000,                           // Amount (50.00 KES)
//!         "tx_67890".to_string(),         // Transaction ID
//!     ).await?;
//!
//!     // Check payment status
//!     let status = collection.status(payment.transaction.id).await?;
//!     
//!     // Refund if needed
//!     if status.transaction.status == "SUCCESS" {
//!         let refund = collection.refund(
//!             payment.transaction.airtel_money_id,
//!             "refund_ref_123".to_string(),
//!         ).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Disbursements (Sending Money)
//!
//! ```rust
//! use airtel_rs::{AirtelMoney, Environment, Country};
//!
//! async fn send_money() -> Result<(), Box<dyn std::error::Error>> {
//!     let airtel = AirtelMoney::new(Environment::Production, Country::Uganda);
//!     let disbursement = airtel.disbursement("client_id".to_string(), "secret".to_string());
//!
//!     let payment = disbursement.disburse(
//!         "256700000000".to_string(),     // Recipient phone
//!         10000,                          // Amount (100.00 UGX)
//!         "disbursement_123".to_string(), // Transaction ID
//!         "salary_payment".to_string(),   // Reference
//!         "1234".to_string(),             // PIN
//!     ).await?;
//!
//!     println!("Disbursement sent: {}", payment.transaction.id);
//!     Ok(())
//! }
//! ```
//!
//! ## Account Operations
//!
//! ```rust
//! use airtel_rs::{AirtelMoney, Environment, Country};
//!
//! async fn check_balance() -> Result<(), Box<dyn std::error::Error>> {
//!     let airtel = AirtelMoney::new(Environment::Sandbox, Country::Tanzania);
//!     let account = airtel.account("client_id".to_string(), "secret".to_string());
//!
//!     let balance = account.get_balance().await?;
//!     println!("Available balance: {} {}", balance.data.balance, balance.data.currency);
//!
//!     // Get account details
//!     let kyc = account.get_kyc("255700000000".to_string()).await?;
//!     println!("Account holder: {} {}", kyc.data.first_name, kyc.data.last_name);
//!
//!     Ok(())
//! }
//! ```

use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

mod client;
mod common;
mod config;
mod enums;
mod errors;
mod products;
mod requests;
mod responses;

// Re-exports for convenient access
pub use client::ApiClient;
pub use config::ProductConfig;
pub use enums::country::Country;
pub use enums::currency::Currency;
pub use enums::environment::Environment;
pub use errors::{AirtelError, AirtelResult};
pub use products::account::Account;
pub use products::cash_in::CashIn;
pub use products::cash_out::CashOut;
pub use products::collection::Collection;
pub use products::disbursement::Disbursement;
pub use products::remittance::{Remittance, PayerInfo};
pub use requests::token_request::TokenRequest;
pub use responses::token_response::TokenResponse;

// Common types re-exports
pub use common::{
    AdditionalInfo, ApiStatus, EnhancedSubscriber, Subscriber, Transaction, TransactionData,
};

/// Main entry point for the Airtel Money SDK
///
/// This struct provides the primary interface for accessing all Airtel Money
/// services. It handles country-specific configuration and provides factory
/// methods for creating product-specific clients.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::{AirtelMoney, Environment, Country};
///
/// // Create SDK instance for Kenya Sandbox environment
/// let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
///
/// // Create product-specific clients
/// let collection = airtel.collection(
///     "your_client_id".to_string(),
///     "your_client_secret".to_string()
/// );
///
/// let disbursement = airtel.disbursement(
///     "your_client_id".to_string(),
///     "your_client_secret".to_string()
/// );
/// ```
#[derive(Debug, Clone)]
pub struct AirtelMoney {
    /// The API environment (Sandbox or Production)
    pub environment: Environment,
    /// The target country for operations
    pub country: Country,
    /// The currency used for the target country
    pub currency: Currency,
}

static ACCESS_TOKEN: Lazy<Arc<Mutex<Option<TokenResponse>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

mod authorization {
    use chrono::Utc;
    use tokio::task;

    use crate::{Environment, TokenResponse, ACCESS_TOKEN};

    async fn create_access_token(
        environment: Environment,
        client_id: &str,
        client_secret: &str,
    ) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let form_data = [
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "client_credentials"),
        ];

        let res = client
            .post(format!("{}/auth/oauth2/token", environment))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "*/*")
            .form(&form_data)
            .send()
            .await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let token_response: TokenResponse = serde_json::from_str(&body)?;
            let cloned = token_response.clone();
            let _t = task::spawn(async move {
                let mut token = ACCESS_TOKEN.lock().await;
                *token = Some(token_response.clone());
            });
            Ok(cloned)
        } else {
            Err(Box::new(std::io::Error::other(res.text().await?)))
        }
    }

    pub async fn get_valid_access_token(
        environment: Environment,
        client_id: &str,
        client_secret: &str,
    ) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let token = ACCESS_TOKEN.lock().await;
        if token.is_some() {
            let token = token.clone().unwrap();
            if token.created_at.is_some() {
                let created_at = token.created_at.unwrap();
                let expires_in = token.expires_in;
                let now = Utc::now();
                let duration = now.signed_duration_since(created_at);
                if duration.num_seconds() < expires_in as i64 {
                    return Ok(token);
                }
                let token: TokenResponse =
                    create_access_token(environment, client_id, client_secret).await?;
                return Ok(token);
            }
        }
        let token: TokenResponse =
            create_access_token(environment, client_id, client_secret).await?;
        Ok(token)
    }
}

impl AirtelMoney {
    /// Creates a new AirtelMoney SDK instance
    ///
    /// This method automatically determines the appropriate currency
    /// for the specified country according to Airtel Money's supported
    /// country-currency mappings.
    ///
    /// # Arguments
    ///
    /// * `environment` - The API environment to use (Sandbox or Production)
    /// * `country` - The target country for operations
    ///
    /// # Returns
    ///
    /// Returns a new AirtelMoney instance configured for the specified
    /// environment and country.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// // Create instance for Kenya Sandbox
    /// let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    /// assert_eq!(airtel.get_currency(), airtel_rs::Currency::KES);
    ///
    /// // Create instance for Uganda Production
    /// let airtel = AirtelMoney::new(Environment::Production, Country::Uganda);
    /// assert_eq!(airtel.get_currency(), airtel_rs::Currency::UGX);
    /// ```
    pub fn new(environment: Environment, country: Country) -> Self {
        let currency = match country {
            Country::Uganda => Currency::UGX,
            Country::Kenya => Currency::KES,
            Country::Tanzania => Currency::TZS,
            Country::Madagascar => Currency::MGA,
            Country::DRC => Currency::CDF,
            Country::Zambia => Currency::ZMW,
            Country::Seychelles => Currency::SCR,
            Country::Rwanda => Currency::RWF,
            Country::Malawi => Currency::MWK,
            Country::Nigeria => Currency::NGN,
            Country::Niger => Currency::XOF,
            Country::Chad => Currency::XAF,
            Country::Gabon => Currency::XAF,
            Country::CongoB => Currency::XAF,
        };
        AirtelMoney {
            environment,
            country,
            currency,
        }
    }

    /// Returns the currency for the configured country
    ///
    /// # Returns
    ///
    /// The currency enum value for the target country.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country, Currency};
    ///
    /// let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    /// assert_eq!(airtel.get_currency(), Currency::KES);
    /// ```
    pub fn get_currency(&self) -> Currency {
        self.currency
    }

    /// Returns the configured country
    ///
    /// # Returns
    ///
    /// The country enum value for this SDK instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// let airtel = AirtelMoney::new(Environment::Production, Country::Uganda);
    /// assert_eq!(airtel.get_country(), Country::Uganda);
    /// ```
    pub fn get_country(&self) -> Country {
        self.country
    }

    /// Creates a Collection client for merchant payment operations
    ///
    /// The Collection service allows merchants to request payments from customers
    /// using USSD push notifications, check payment status, and process refunds.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth2 client ID for API authentication
    /// * `client_secret` - OAuth2 client secret for API authentication
    ///
    /// # Returns
    ///
    /// A Collection client configured with the SDK's environment and country settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    /// let collection = airtel.collection(
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string()
    /// );
    ///
    /// // Use collection for payment operations
    /// // let payment = collection.ussd_push(...).await?;
    /// ```
    pub fn collection(&self, client_id: String, client_secret: String) -> Collection {
        Collection::new(
            self.country,
            self.currency,
            self.environment,
            client_id,
            client_secret,
        )
    }

    /// Creates a Disbursement client for sending money to customers
    ///
    /// The Disbursement service allows businesses to send money directly
    /// to customer mobile wallets, such as salary payments, refunds, or rewards.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth2 client ID for API authentication
    /// * `client_secret` - OAuth2 client secret for API authentication
    ///
    /// # Returns
    ///
    /// A Disbursement client configured with the SDK's environment and country settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// let airtel = AirtelMoney::new(Environment::Production, Country::Uganda);
    /// let disbursement = airtel.disbursement(
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string()
    /// );
    ///
    /// // Use disbursement to send money
    /// // let payment = disbursement.disburse(...).await?;
    /// ```
    pub fn disbursement(&self, client_id: String, client_secret: String) -> Disbursement {
        Disbursement::new(
            self.country,
            self.currency,
            self.environment,
            client_id,
            client_secret,
        )
    }

    /// Creates a Remittance client for cross-border money transfers
    ///
    /// The Remittance service enables international money transfers
    /// between supported Airtel Money countries and external financial services.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth2 client ID for API authentication
    /// * `client_secret` - OAuth2 client secret for API authentication
    ///
    /// # Returns
    ///
    /// A Remittance client configured with the SDK's environment and country settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    /// let remittance = airtel.remittance(
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string()
    /// );
    ///
    /// // Use remittance for international transfers
    /// // let transfer = remittance.send(...).await?;
    /// ```
    pub fn remittance(&self, client_id: String, client_secret: String) -> Remittance {
        Remittance::new(
            self.country,
            self.currency,
            self.environment,
            client_id,
            client_secret,
        )
    }

    /// Creates a CashIn client for depositing money to customer wallets
    ///
    /// The CashIn service allows businesses to add money to customer
    /// Airtel Money wallets, typically used for agent operations or
    /// account top-ups.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth2 client ID for API authentication
    /// * `client_secret` - OAuth2 client secret for API authentication
    ///
    /// # Returns
    ///
    /// A CashIn client configured with the SDK's environment and country settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// let airtel = AirtelMoney::new(Environment::Sandbox, Country::Tanzania);
    /// let cash_in = airtel.cash_in(
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string()
    /// );
    ///
    /// // Use cash_in to deposit money
    /// // let deposit = cash_in.cash_in(...).await?;
    /// ```
    pub fn cash_in(&self, client_id: String, client_secret: String) -> CashIn {
        CashIn::new(
            self.country,
            self.currency,
            self.environment,
            client_id,
            client_secret,
        )
    }

    /// Creates a CashOut client for withdrawing money from customer wallets
    ///
    /// The CashOut service allows businesses to withdraw money from customer
    /// Airtel Money wallets, typically used for agent operations or cash
    /// withdrawal services.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth2 client ID for API authentication
    /// * `client_secret` - OAuth2 client secret for API authentication
    ///
    /// # Returns
    ///
    /// A CashOut client configured with the SDK's environment and country settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// let airtel = AirtelMoney::new(Environment::Production, Country::Zambia);
    /// let cash_out = airtel.cash_out(
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string()
    /// );
    ///
    /// // Use cash_out to withdraw money
    /// // let withdrawal = cash_out.cash_out(...).await?;
    /// ```
    pub fn cash_out(&self, client_id: String, client_secret: String) -> CashOut {
        CashOut::new(
            self.country,
            self.currency,
            self.environment,
            client_id,
            client_secret,
        )
    }

    /// Creates an Account client for balance and KYC operations
    ///
    /// The Account service provides access to account information including
    /// balance inquiries, KYC (Know Your Customer) data, and transaction history.
    ///
    /// # Arguments
    ///
    /// * `client_id` - OAuth2 client ID for API authentication
    /// * `client_secret` - OAuth2 client secret for API authentication
    ///
    /// # Returns
    ///
    /// An Account client configured with the SDK's environment and country settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{AirtelMoney, Environment, Country};
    ///
    /// let airtel = AirtelMoney::new(Environment::Sandbox, Country::Rwanda);
    /// let account = airtel.account(
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string()
    /// );
    ///
    /// // Use account for balance and KYC operations
    /// // let balance = account.get_balance().await?;
    /// // let kyc = account.get_kyc("250700000000".to_string()).await?;
    /// ```
    pub fn account(&self, client_id: String, client_secret: String) -> Account {
        Account::new(
            self.country,
            self.currency,
            self.environment,
            client_id,
            client_secret,
        )
    }
}
