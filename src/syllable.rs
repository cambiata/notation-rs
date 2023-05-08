use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Eq, Hash)]
pub enum SyllableType {
    Text(String),
    TextWithHyphen(String),
    Hyphen,
    Extension(i32), // length
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Syllable {
    syllable_type: SyllableType,
}

impl Syllable {
    fn new(syllable_type: SyllableType) -> Self {
        Self { syllable_type }
    }
}
