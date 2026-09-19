/*
Adapter error types for native EDA file conversion.

Adapters return structured failures through this shared type so the browser,
CLI, and future integrations can report errors consistently.
*/

use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdapterError {
    UnsupportedFormat(String),
    InvalidSyntax(String),
    MissingField(String),
}

impl Display for AdapterError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedFormat(message)
            | Self::InvalidSyntax(message)
            | Self::MissingField(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for AdapterError {}
