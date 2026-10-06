use std::io::Read;

use crate::{MCBoolean, MCPrefixedArray, MCString, MCType, MCUuid, ProtocolError};

/// A player identity and its bounded profile properties (protocol 777).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameProfile {
    pub uuid: MCUuid,
    pub username: MCString<16>,
    pub properties: MCPrefixedArray<ProfileProperty, 16>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_signed_and_unsigned_property_bytes() {
        for (signature, expected) in [
            (None, vec![1, b'n', 1, b'v', 0]),
            (Some("s".into()), vec![1, b'n', 1, b'v', 1, 1, b's']),
        ] {
            let property = ProfileProperty {
                name: "n".into(),
                value: "v".into(),
                signature,
            };
            assert_eq!(property.pack().unwrap(), expected);
            assert_eq!(
                ProfileProperty::unpack(&mut expected.as_slice()).unwrap(),
                property
            );
        }
    }

    #[test]
    fn rejects_oversized_property_counts() {
        let mut bytes = vec![0; 16];
        bytes.extend([1, b'a', 17]);
        assert!(matches!(
            GameProfile::unpack(&mut bytes.as_slice()),
            Err(ProtocolError::Overflow)
        ));
    }
}

/// A profile property, optionally signed by the authentication service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileProperty {
    pub name: MCString<64>,
    pub value: MCString,
    pub signature: Option<MCString<1024>>,
}

impl MCType for GameProfile {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.uuid.pack()?;
        bytes.extend(self.username.pack()?);
        bytes.extend(self.properties.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            uuid: MCUuid::unpack(source)?,
            username: MCString::unpack(source)?,
            properties: MCPrefixedArray::unpack(source)?,
        })
    }
}

impl MCType for ProfileProperty {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.name.pack()?;
        bytes.extend(self.value.pack()?);
        bytes.extend(MCBoolean(self.signature.is_some()).pack()?);
        if let Some(signature) = &self.signature {
            bytes.extend(signature.pack()?);
        }
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            name: MCString::unpack(source)?,
            value: MCString::unpack(source)?,
            signature: if MCBoolean::unpack(source)?.0 {
                Some(MCString::unpack(source)?)
            } else {
                None
            },
        })
    }
}
