use std::io::Read;

use crate::{MCPrefixedArray, MCString, MCType, ProtocolError};

/// A pack identity used to negotiate client-local registry values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownPack {
    pub namespace: MCString,
    pub id: MCString,
    pub version: MCString,
}

/// Known Packs has the same body in both directions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownPacks {
    /// Local codec bound; sufficient for this server's single vanilla pack.
    pub packs: MCPrefixedArray<KnownPack, 64>,
}

impl MCType for KnownPack {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.namespace.pack()?;
        bytes.extend(self.id.pack()?);
        bytes.extend(self.version.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            namespace: MCString::unpack(source)?,
            id: MCString::unpack(source)?,
            version: MCString::unpack(source)?,
        })
    }
}

impl MCType for KnownPacks {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        self.packs.pack()
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            packs: MCPrefixedArray::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_pack_identity_wire_fields() {
        let packs = KnownPacks {
            packs: vec![KnownPack {
                namespace: "n".into(),
                id: "i".into(),
                version: "v".into(),
            }]
            .into(),
        };
        let bytes = [1, 1, b'n', 1, b'i', 1, b'v'];
        assert_eq!(packs.pack().unwrap(), bytes);
        assert_eq!(KnownPacks::unpack(&mut bytes.as_slice()).unwrap(), packs);
        assert!(matches!(
            KnownPacks::unpack(&mut [65].as_slice()),
            Err(ProtocolError::Overflow)
        ));
    }
}
