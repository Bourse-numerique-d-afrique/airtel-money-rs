use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    UGX,
    KES,
    TZS,
    MGA,
    CDF,
    XAF,
    ZMW,
    XOF,
    RWF,
    MWK,
    SCR,
    NGN,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Currency::UGX => write!(f, "UGX"),
            Currency::KES => write!(f, "KES"),
            Currency::TZS => write!(f, "TZS"),
            Currency::MGA => write!(f, "MGA"),
            Currency::CDF => write!(f, "CDF"),
            Currency::XAF => write!(f, "XAF"),
            Currency::ZMW => write!(f, "ZMW"),
            Currency::XOF => write!(f, "XOF"),
            Currency::RWF => write!(f, "RWF"),
            Currency::MWK => write!(f, "MWK"),
            Currency::SCR => write!(f, "SCR"),
            Currency::NGN => write!(f, "NGN"),
        }
    }
}
