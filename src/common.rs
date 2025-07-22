use crate::{Country, Currency};
/// Common types and structures shared across request/response modules
///
/// This module contains shared data structures that are used by multiple
/// request and response types to eliminate code duplication and ensure
/// consistency across the API.
use serde::{Deserialize, Serialize};

/// Common subscriber information used in various API requests
///
/// This structure represents a subscriber (customer) with their mobile number
/// and is used across multiple API endpoints for collections, disbursements,
/// and other operations.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::common::Subscriber;
///
/// let subscriber = Subscriber {
///     msisdn: "254700000000".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    /// Mobile Subscriber ISDN (phone number) in international format
    pub msisdn: String,
}

impl Subscriber {
    /// Creates a new Subscriber instance
    ///
    /// # Arguments
    ///
    /// * `msisdn` - The mobile number in international format (e.g., "254700000000")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::Subscriber;
    ///
    /// let subscriber = Subscriber::new("254700000000".to_string());
    /// ```
    pub fn new(msisdn: String) -> Self {
        Self { msisdn }
    }
}

/// Common transaction information used in various API requests
///
/// This structure represents transaction details including amount and
/// unique identifier, used across multiple API endpoints.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::common::Transaction;
///
/// let transaction = Transaction {
///     amount: 1000,
///     id: "tx_12345".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction amount in the smallest currency unit (e.g., cents)
    pub amount: i32,
    /// Unique transaction identifier
    pub id: String,
}

impl Transaction {
    /// Creates a new Transaction instance
    ///
    /// # Arguments
    ///
    /// * `amount` - Transaction amount in smallest currency unit
    /// * `id` - Unique transaction identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::Transaction;
    ///
    /// let transaction = Transaction::new(1000, "tx_12345".to_string());
    /// ```
    pub fn new(amount: i32, id: String) -> Self {
        Self { amount, id }
    }
}

/// Additional information key-value pairs for API requests
///
/// This structure allows passing additional metadata with API requests,
/// commonly used for remarks, references, or other supplementary data.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::common::AdditionalInfo;
///
/// let info = AdditionalInfo {
///     key: "remark".to_string(),
///     value: "Payment for services".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalInfo {
    /// The key/name of the additional information
    pub key: String,
    /// The value of the additional information
    pub value: String,
}

impl AdditionalInfo {
    /// Creates a new AdditionalInfo instance
    ///
    /// # Arguments
    ///
    /// * `key` - The key/name for the additional information
    /// * `value` - The value of the additional information
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::AdditionalInfo;
    ///
    /// let info = AdditionalInfo::new("remark", "Payment for services");
    /// ```
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    /// Creates a remark additional info entry
    ///
    /// # Arguments
    ///
    /// * `remark` - The remark text
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::AdditionalInfo;
    ///
    /// let remark = AdditionalInfo::remark("Payment for goods");
    /// ```
    pub fn remark(remark: &str) -> Self {
        Self::new("remark", remark)
    }
}

/// Common status information returned by API responses
///
/// This structure represents the standard status information returned
/// by Airtel Money API responses, providing details about the operation
/// success, error codes, and descriptive messages.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::common::ApiStatus;
///
/// let status = ApiStatus {
///     code: "200".to_string(),
///     message: "SUCCESS".to_string(),
///     result_code: "ESB000010".to_string(),
///     response_code: "DP00800001006".to_string(),
///     success: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStatus {
    /// HTTP status code as string
    pub code: String,
    /// Human-readable status message
    pub message: String,
    /// Internal result code from Airtel Money system
    pub result_code: String,
    /// API-specific response code
    pub response_code: String,
    /// Boolean indicating if the operation was successful
    pub success: bool,
}

impl ApiStatus {
    /// Checks if the API call was successful
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::ApiStatus;
    ///
    /// let status = ApiStatus {
    ///     code: "200".to_string(),
    ///     message: "SUCCESS".to_string(),
    ///     result_code: "ESB000010".to_string(),
    ///     response_code: "DP00800001006".to_string(),
    ///     success: true,
    /// };
    ///
    /// assert!(status.is_success());
    /// ```
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Returns true if this is an authentication/authorization error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::ApiStatus;
    ///
    /// let status = ApiStatus {
    ///     code: "401".to_string(),
    ///     message: "Unauthorized".to_string(),
    ///     result_code: "".to_string(),
    ///     response_code: "".to_string(),
    ///     success: false,
    /// };
    ///
    /// assert!(status.is_auth_error());
    /// ```
    pub fn is_auth_error(&self) -> bool {
        self.code == "401" || self.message.to_lowercase().contains("unauthorized")
    }
}

/// Enhanced subscriber information with country and currency details
///
/// This structure extends the basic subscriber information with additional
/// context about the subscriber's location and preferred currency.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::common::EnhancedSubscriber;
/// use airtel_rs::{Country, Currency};
///
/// let subscriber = EnhancedSubscriber {
///     country: Country::Kenya,
///     msisdn: "254700000000".to_string(),
///     currency: Currency::KES,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedSubscriber {
    /// Subscriber's country
    pub country: Country,
    /// Mobile Subscriber ISDN (phone number)
    pub msisdn: String,
    /// Subscriber's currency
    pub currency: Currency,
}

impl EnhancedSubscriber {
    /// Creates a new EnhancedSubscriber instance
    ///
    /// # Arguments
    ///
    /// * `country` - Subscriber's country
    /// * `msisdn` - Mobile number in international format
    /// * `currency` - Subscriber's currency
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::EnhancedSubscriber;
    /// use airtel_rs::{Country, Currency};
    ///
    /// let subscriber = EnhancedSubscriber::new(
    ///     Country::Kenya,
    ///     "254700000000".to_string(),
    ///     Currency::KES,
    /// );
    /// ```
    pub fn new(country: Country, msisdn: String, currency: Currency) -> Self {
        Self {
            country,
            msisdn,
            currency,
        }
    }
}

/// Common transaction data returned in API responses
///
/// This structure represents transaction information returned by the API,
/// including identifiers, status, and amounts.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::common::TransactionData;
///
/// let transaction = TransactionData {
///     reference_id: "ref_123".to_string(),
///     airtel_money_id: "AM_456".to_string(),
///     id: "tx_789".to_string(),
///     status: "SUCCESS".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    /// Reference ID for the transaction
    pub reference_id: String,
    /// Airtel Money system transaction ID
    pub airtel_money_id: String,
    /// Client-provided transaction ID
    pub id: String,
    /// Current status of the transaction
    pub status: String,
}

impl TransactionData {
    /// Checks if the transaction was successful
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::TransactionData;
    ///
    /// let transaction = TransactionData {
    ///     reference_id: "ref_123".to_string(),
    ///     airtel_money_id: "AM_456".to_string(),
    ///     id: "tx_789".to_string(),
    ///     status: "SUCCESS".to_string(),
    /// };
    ///
    /// assert!(transaction.is_successful());
    /// ```
    pub fn is_successful(&self) -> bool {
        self.status.to_uppercase() == "SUCCESS"
    }

    /// Checks if the transaction is pending
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::TransactionData;
    ///
    /// let transaction = TransactionData {
    ///     reference_id: "ref_123".to_string(),
    ///     airtel_money_id: "AM_456".to_string(),
    ///     id: "tx_789".to_string(),
    ///     status: "PENDING".to_string(),
    /// };
    ///
    /// assert!(transaction.is_pending());
    /// ```
    pub fn is_pending(&self) -> bool {
        self.status.to_uppercase() == "PENDING"
    }

    /// Checks if the transaction failed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::common::TransactionData;
    ///
    /// let transaction = TransactionData {
    ///     reference_id: "ref_123".to_string(),
    ///     airtel_money_id: "AM_456".to_string(),
    ///     id: "tx_789".to_string(),
    ///     status: "FAILED".to_string(),
    /// };
    ///
    /// assert!(transaction.is_failed());
    /// ```
    pub fn is_failed(&self) -> bool {
        self.status.to_uppercase() == "FAILED" || self.status.to_uppercase() == "ERROR"
    }
}
