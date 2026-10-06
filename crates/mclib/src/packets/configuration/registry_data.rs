use std::io::Read;

use crate::nbt::Nbt;
use crate::{MCBoolean, MCPrefixedArray, MCString, MCType, ProtocolError};

/// One synchronized registry, with stable entry order defining numeric IDs.
#[derive(Debug, Clone, PartialEq)]
pub struct RegistryData {
    pub registry_id: MCString,
    /// Local safety bound for protocol decoding, not a protocol-wide limit.
    pub entries: MCPrefixedArray<RegistryEntry, 4096>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegistryEntry {
    pub id: MCString,
    /// Absent only when the client knows the pack providing this entry.
    pub data: Option<Nbt>,
}

impl MCType for RegistryEntry {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.id.pack()?;
        bytes.extend(MCBoolean(self.data.is_some()).pack()?);
        if let Some(data) = &self.data {
            bytes.extend(data.encode_network()?);
        }
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            id: MCString::unpack(source)?,
            data: if MCBoolean::unpack(source)?.0 {
                Some(Nbt::decode_network(source)?)
            } else {
                None
            },
        })
    }
}

impl MCType for RegistryData {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.registry_id.pack()?;
        bytes.extend(self.entries.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            registry_id: MCString::unpack(source)?,
            entries: MCPrefixedArray::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_pack_backed_and_inline_values() {
        let mut entry = RegistryEntry {
            id: "a".into(),
            data: None,
        };
        assert_eq!(entry.pack().unwrap(), [1, b'a', 0]);
        entry.data = Some(Nbt::Int(42));
        let bytes = [1, b'a', 1, 3, 0, 0, 0, 42];
        assert_eq!(entry.pack().unwrap(), bytes);
        assert_eq!(RegistryEntry::unpack(&mut bytes.as_slice()).unwrap(), entry);
    }

    #[test]
    fn rejects_oversized_entry_counts_before_reading_entries() {
        assert!(matches!(
            RegistryData::unpack(&mut [1, b'r', 0x81, 0x20].as_slice()),
            Err(ProtocolError::Overflow)
        ));
    }
}
