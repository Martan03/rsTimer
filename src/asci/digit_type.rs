use serde::{Deserialize, Serialize};

/// Contains all supported digits types (fonts)
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitType {
    #[default]
    Italic,
    Train,
    Card,
}

impl DigitType {
    /// Gets next digit type
    pub fn next(&self) -> DigitType {
        match self {
            DigitType::Italic => DigitType::Train,
            DigitType::Train => DigitType::Card,
            DigitType::Card => DigitType::Italic,
        }
    }

    /// Gets previous digit type
    pub fn prev(&self) -> DigitType {
        match self {
            DigitType::Italic => DigitType::Card,
            DigitType::Train => DigitType::Italic,
            DigitType::Card => DigitType::Train,
        }
    }
}
