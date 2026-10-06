use std::io::Read;

use crate::{MCPrefixedArray, MCString, MCType, ProtocolError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFlags {
    pub flags: MCPrefixedArray<MCString, 64>,
}

impl MCType for FeatureFlags {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        self.flags.pack()
    }
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            flags: MCPrefixedArray::unpack(source)?,
        })
    }
}
