//! Error categories shared by all protocol codecs.

use std::{fmt, io};

/// An error encountered while encoding or decoding protocol data.
#[derive(Debug)]
pub enum ProtocolError {
    /// Reading or writing protocol data failed, including truncated input.
    Io(io::Error),
    /// A value, length, or nesting depth exceeds its supported bounds.
    Overflow,
    /// Data contains an invalid value, encoding, or structure.
    InvalidData,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "protocol I/O error: {error}"),
            Self::Overflow => formatter.write_str("protocol data exceeds supported bounds"),
            Self::InvalidData => formatter.write_str("invalid protocol data"),
        }
    }
}

impl std::error::Error for ProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Overflow | Self::InvalidData => None,
        }
    }
}

impl From<io::Error> for ProtocolError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
