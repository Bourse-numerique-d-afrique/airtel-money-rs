/// Configuration module for shared product configuration and settings
use crate::{AirtelError, AirtelResult, Country, Currency, Environment};
use std::env;

/// Shared configuration for all Airtel Money product modules
///
/// This struct contains the common configuration parameters needed
/// by all product modules (Collection, Disbursement, etc.) to eliminate
/// code duplication and provide a consistent configuration interface.
///
/// # Examples
///
/// ```rust
/// use airtel_rs::{ProductConfig, Environment, Country, Currency};
///
/// let config = ProductConfig::new(
///     Environment::Sandbox,
///     Country::Kenya,
///     Currency::KES,
///     "client_id".to_string(),
///     "client_secret".to_string(),
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ProductConfig {
    /// The target country for API operations
    pub country: Country,
    /// The currency used for transactions in the target country
    pub currency: Currency,
    /// The API environment (Sandbox or Production)
    pub environment: Environment,
    /// OAuth2 client ID for API authentication
    pub client_id: String,
    /// OAuth2 client secret for API authentication
    pub client_secret: String,
}

impl ProductConfig {
    /// Creates a new ProductConfig instance
    ///
    /// # Arguments
    ///
    /// * `environment` - The API environment to use
    /// * `country` - The target country for operations
    /// * `currency` - The currency for transactions
    /// * `client_id` - OAuth2 client ID
    /// * `client_secret` - OAuth2 client secret
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{ProductConfig, Environment, Country, Currency};
    ///
    /// let config = ProductConfig::new(
    ///     Environment::Sandbox,
    ///     Country::Kenya,
    ///     Currency::KES,
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string(),
    /// );
    /// ```
    pub fn new(
        environment: Environment,
        country: Country,
        currency: Currency,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        }
    }

    /// Returns the API base URL for the configured environment
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airtel_rs::{ProductConfig, Environment, Country, Currency};
    ///
    /// let config = ProductConfig::new(
    ///     Environment::Sandbox,
    ///     Country::Kenya,
    ///     Currency::KES,
    ///     "client_id".to_string(),
    ///     "client_secret".to_string(),
    /// );
    ///
    /// assert_eq!(config.base_url(), "https://openapiuat.airtel.africa");
    /// ```
    pub fn base_url(&self) -> &'static str {
        match self.environment {
            Environment::Sandbox => "https://openapiuat.airtel.africa",
            Environment::Production => "https://openapi.airtel.africa",
        }
    }

    /// Returns the country code as a string
    ///
    /// Used for setting the X-Country header in API requests
    pub fn country_code(&self) -> String {
        self.country.to_string()
    }

    /// Returns the currency code as a string
    ///
    /// Used for setting the X-Currency header in API requests
    pub fn currency_code(&self) -> String {
        self.currency.to_string()
    }

    /// Loads configuration from environment variables
    ///
    /// Reads required environment variables and validates their presence.
    /// Required variables:
    /// - AIRTEL_ENVIRONMENT: "sandbox" or "production"
    /// - AIRTEL_COUNTRY: ISO 2-letter country code
    /// - AIRTEL_CLIENT_ID: OAuth2 client ID
    /// - AIRTEL_CLIENT_SECRET: OAuth2 client secret
    ///
    /// # Errors
    ///
    /// Returns `AirtelError::MissingEnvironmentVariable` if any required variable is missing,
    /// or `AirtelError::ConfigurationError` if values are invalid.
    pub fn from_env() -> AirtelResult<Self> {
        dotenvy::dotenv().ok();

        let environment = env::var("AIRTEL_ENVIRONMENT")
            .map_err(|_| AirtelError::MissingEnvironmentVariable {
                variable: "AIRTEL_ENVIRONMENT".to_string(),
            })?
            .parse()?;

        let country = env::var("AIRTEL_COUNTRY")
            .map_err(|_| AirtelError::MissingEnvironmentVariable {
                variable: "AIRTEL_COUNTRY".to_string(),
            })?
            .parse()?;

        let currency = Country::currency_for(&country)?;

        let client_id =
            env::var("AIRTEL_CLIENT_ID").map_err(|_| AirtelError::MissingEnvironmentVariable {
                variable: "AIRTEL_CLIENT_ID".to_string(),
            })?;

        let client_secret = env::var("AIRTEL_CLIENT_SECRET").map_err(|_| {
            AirtelError::MissingEnvironmentVariable {
                variable: "AIRTEL_CLIENT_SECRET".to_string(),
            }
        })?;

        Ok(Self {
            country,
            currency,
            environment,
            client_id,
            client_secret,
        })
    }
}

impl std::str::FromStr for Environment {
    type Err = AirtelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sandbox" => Ok(Environment::Sandbox),
            "production" => Ok(Environment::Production),
            _ => Err(AirtelError::ConfigurationError {
                message: format!("Invalid environment: {}. Use 'sandbox' or 'production'", s),
            }),
        }
    }
}

impl std::str::FromStr for Country {
    type Err = AirtelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "KE" => Ok(Country::Kenya),
            "UG" => Ok(Country::Uganda),
            "TZ" => Ok(Country::Tanzania),
            "MG" => Ok(Country::Madagascar),
            "CD" => Ok(Country::DRC),
            "ZM" => Ok(Country::Zambia),
            "SC" => Ok(Country::Seychelles),
            "RW" => Ok(Country::Rwanda),
            "MW" => Ok(Country::Malawi),
            "NG" => Ok(Country::Nigeria),
            "NE" => Ok(Country::Niger),
            "TD" => Ok(Country::Chad),
            "GA" => Ok(Country::Gabon),
            "CG" => Ok(Country::CongoB),
            _ => Err(AirtelError::ConfigurationError {
                message: format!("Invalid country code: {}", s),
            }),
        }
    }
}
