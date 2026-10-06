use std::io::Read;

use crate::{MCShort, MCType, MCUnsignedByte, MCVarInt, ProtocolError};

/// All-air, fluid-free section with a single biome, for protocol 777.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyChunkSection {
    pub biome: MCVarInt,
}

impl MCType for EmptyChunkSection {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        if self.biome.0 < 0 {
            return Err(ProtocolError::InvalidData);
        }
        // Two short counts, zero palette bits, air state ID 0, zero biome bits.
        // Single-value palettes have no storage words or array length prefix.
        let mut bytes = vec![0; 7];
        bytes.extend(self.biome.pack()?);
        Ok(bytes)
    }
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        if MCShort::unpack(source)?.0 != 0
            || MCShort::unpack(source)?.0 != 0
            || MCUnsignedByte::unpack(source)?.0 != 0
            || MCVarInt::unpack(source)?.0 != 0
            || MCUnsignedByte::unpack(source)?.0 != 0
        {
            return Err(ProtocolError::InvalidData);
        }
        let biome = MCVarInt::unpack(source)?;
        if biome.0 < 0 {
            return Err(ProtocolError::InvalidData);
        }
        Ok(Self { biome })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_26_3_section_layout() {
        let section = EmptyChunkSection { biome: 59.into() };
        let bytes = [0, 0, 0, 0, 0, 0, 0, 59];
        assert_eq!(section.pack().unwrap(), bytes);
        assert_eq!(
            EmptyChunkSection::unpack(&mut bytes.as_slice()).unwrap(),
            section
        );
        assert!(EmptyChunkSection::unpack(&mut &[0, 1][..]).is_err());
    }
}
