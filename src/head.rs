use crate::core::Accidental;
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Head {
    pub level: i8,
    pub attr: HeadAttributes,
}

impl Head {
    pub fn new(level: i8, attr: HeadAttributes) -> Head {
        Head { level, attr }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct HeadAttributes {
    pub accidental: Option<Accidental>,
}
