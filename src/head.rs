use crate::core::Accidental;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Head {
    pub level: i8,
    pub accidental: Option<Accidental>,
    pub attr: HeadAttributes,
}

impl Head {
    pub fn new(level: i8, accidental: Option<Accidental>, attr: HeadAttributes) -> Head {
        Head {
            level,
            accidental,
            attr,
        }
    }

    pub fn from_level(level: i8) -> Head {
        Head {
            level,
            accidental: None,
            attr: HeadAttributes {},
        }
    }

    pub fn from_level_and_accidental(level: i8, accidental: Accidental) -> Head {
        Head {
            level,
            accidental: Some(accidental),
            attr: HeadAttributes {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct HeadAttributes {
    // pub accidental: Option<Accidental>,
}
