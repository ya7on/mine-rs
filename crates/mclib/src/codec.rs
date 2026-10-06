//! The encoding contract shared by all protocol types and packets.

use std::io::Read;

use crate::ProtocolError;

/// A value encoded using its Minecraft protocol representation.
pub trait MCType: Sized {
    /// Encodes the value using its Minecraft protocol representation.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError`] when the value violates an encoding bound.
    fn pack(&self) -> Result<Vec<u8>, ProtocolError>;

    /// Decodes the value from its Minecraft protocol representation.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError`] when the source ends prematurely or carries
    /// data that violates a decoding bound.
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError>;
}
