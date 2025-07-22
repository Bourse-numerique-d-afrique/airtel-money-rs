use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Country {
    Uganda,
    Kenya,
    Tanzania,
    Madagascar,
    DRC,
    Gabon,
    Zambia,
    Niger,
    Chad,
    Rwanda,
    Malawi,
    CongoB,
    Seychelles,
    Nigeria,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Country::Uganda => write!(f, "UG"),
            Country::Kenya => write!(f, "KE"),
            Country::Tanzania => write!(f, "TZ"),
            Country::Madagascar => write!(f, "MG"),
            Country::DRC => write!(f, "CD"),
            Country::Gabon => write!(f, "GA"),
            Country::Zambia => write!(f, "ZM"),
            Country::Niger => write!(f, "NE"),
            Country::Chad => write!(f, "TD"),
            Country::Rwanda => write!(f, "RW"),
            Country::Malawi => write!(f, "MW"),
            Country::CongoB => write!(f, "CG"),
            Country::Seychelles => write!(f, "SC"),
            Country::Nigeria => write!(f, "NG"),
        }
    }
}
