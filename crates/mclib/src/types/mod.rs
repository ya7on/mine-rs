//! Minecraft protocol field types.

use std::fmt;
use std::io::{self, Read};

mod long;
mod string;
mod var_int;

pub use long::MCLong;
pub use string::MCString;
pub use var_int::MCVarInt;

/// A value encoded using its Minecraft protocol representation.
pub trait MCType: Sized {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError>;
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError>;
}

/// An error encountered while encoding or decoding protocol data.
#[derive(Debug)]
pub enum ProtocolError {
    Io(io::Error),
    VarIntTooLong,
    PacketLengthVarIntTooLong,
    NegativeLength(i32),
    InvalidPacketLength(i32),
    InvalidPacketId(i32),
    PacketTooLarge {
        length: usize,
        max_length: usize,
    },
    InvalidUtf8,
    StringTooLong {
        characters: usize,
        max_characters: usize,
    },
    EncodedStringTooLong {
        bytes: usize,
        max_bytes: usize,
    },
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error while reading protocol data: {error}"),
            Self::VarIntTooLong => formatter.write_str("VarInt is longer than five bytes"),
            Self::PacketLengthVarIntTooLong => {
                formatter.write_str("packet length VarInt is longer than three bytes")
            }
            Self::NegativeLength(length) => write!(formatter, "negative length: {length}"),
            Self::InvalidPacketLength(length) => {
                write!(formatter, "invalid packet length: {length}")
            }
            Self::InvalidPacketId(packet_id) => {
                write!(formatter, "invalid negative packet ID: {packet_id}")
            }
            Self::PacketTooLarge { length, max_length } => write!(
                formatter,
                "packet length is {length} bytes, maximum is {max_length}"
            ),
            Self::InvalidUtf8 => formatter.write_str("string is not valid UTF-8"),
            Self::StringTooLong {
                characters,
                max_characters,
            } => write!(
                formatter,
                "string contains {characters} characters, maximum is {max_characters}"
            ),
            Self::EncodedStringTooLong { bytes, max_bytes } => write!(
                formatter,
                "encoded string contains {bytes} bytes, maximum is {max_bytes}"
            ),
        }
    }
}

impl std::error::Error for ProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ProtocolError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
