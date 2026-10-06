use std::io::Read;

use crate::{MCPrefixedArray, MCString, MCType, MCVarInt, ProtocolError};

/// Registry tags with numeric entry IDs defined by Registry Data order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateTags {
    pub registries: MCPrefixedArray<RegistryTags, 64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryTags {
    pub registry_id: MCString,
    pub tags: MCPrefixedArray<Tag, 4096>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub name: MCString,
    pub entries: MCPrefixedArray<MCVarInt, 4096>,
}

impl MCType for UpdateTags {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        self.registries.pack()
    }
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            registries: MCPrefixedArray::unpack(source)?,
        })
    }
}

impl MCType for RegistryTags {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.registry_id.pack()?;
        bytes.extend(self.tags.pack()?);
        Ok(bytes)
    }
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            registry_id: MCString::unpack(source)?,
            tags: MCPrefixedArray::unpack(source)?,
        })
    }
}

impl MCType for Tag {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.name.pack()?;
        bytes.extend(self.entries.pack()?);
        Ok(bytes)
    }
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            name: MCString::unpack(source)?,
            entries: MCPrefixedArray::unpack(source)?,
        })
    }
}
