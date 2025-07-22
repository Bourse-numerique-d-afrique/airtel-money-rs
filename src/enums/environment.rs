use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Sandbox => write!(f, "https://openapiuat.airtel.africa/"),
            Environment::Production => write!(f, "https://openapi.airtel.africa/"),
        }
    }
}
