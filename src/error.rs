use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotationError {
    Generic(String),
    ComplexError(String),
    QuickCodeError(String),
    // UnknownElement(String),
    // UnknownAttribute(String),
    // TextfieldEmpty(String),
}

impl std::fmt::Display for NotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotationError::Generic(s) => write!(f, "NotationError::Generic error: {}", s),
            NotationError::QuickCodeError(s) => {
                write!(f, "NotationError::QuickCodeError error: {}", s)
            }
            NotationError::ComplexError(s) => {
                write!(f, "NotationError::QuickCodeError error: {}", s)
            }
            // NotationError::UnknownElement(s) => write!(f, "NotationError::UnknownElement: {}", s),
            // NotationError::UnknownAttribute(s) => {
            //     write!(f, "NotationError::UnknownAttribute: {}", s)
            // }
            // NotationError::TextfieldEmpty(s) => write!(f, "MusicXmlError::TextfieldEmpty: {}", s),
        }
    }
}
