use std::io::Read;

use crate::{MCBitSet, MCByteArray, MCInt, MCPrefixedArray, MCType, ProtocolError};

/// Carries encoded sections, heightmaps, block entities and light for protocol 777.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkDataAndUpdateLight {
    pub x: MCInt,
    pub z: MCInt,
    pub heightmaps: MCPrefixedArray<Heightmap, 6>,
    pub data: MCByteArray<1_048_576>,
    pub block_entities: MCPrefixedArray<BlockEntity, 4096>,
    pub sky_light_mask: MCBitSet,
    pub block_light_mask: MCBitSet,
    pub empty_sky_light_mask: MCBitSet,
    pub empty_block_light_mask: MCBitSet,
    pub sky_light: MCPrefixedArray<MCByteArray<2048>, 64>,
    pub block_light: MCPrefixedArray<MCByteArray<2048>, 64>,
}

impl MCType for ChunkDataAndUpdateLight {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.x.pack()?);
        bytes.extend(self.z.pack()?);
        bytes.extend(self.heightmaps.pack()?);
        bytes.extend(self.data.pack()?);
        bytes.extend(self.block_entities.pack()?);
        bytes.extend(self.sky_light_mask.pack()?);
        bytes.extend(self.block_light_mask.pack()?);
        bytes.extend(self.empty_sky_light_mask.pack()?);
        bytes.extend(self.empty_block_light_mask.pack()?);
        bytes.extend(self.sky_light.pack()?);
        bytes.extend(self.block_light.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            x: MCInt::unpack(source)?,
            z: MCInt::unpack(source)?,
            heightmaps: MCPrefixedArray::<Heightmap, 6>::unpack(source)?,
            data: MCByteArray::<1_048_576>::unpack(source)?,
            block_entities: MCPrefixedArray::<BlockEntity, 4096>::unpack(source)?,
            sky_light_mask: MCBitSet::unpack(source)?,
            block_light_mask: MCBitSet::unpack(source)?,
            empty_sky_light_mask: MCBitSet::unpack(source)?,
            empty_block_light_mask: MCBitSet::unpack(source)?,
            sky_light: MCPrefixedArray::<MCByteArray<2048>, 64>::unpack(source)?,
            block_light: MCPrefixedArray::<MCByteArray<2048>, 64>::unpack(source)?,
        })
    }
}

/// Numeric heightmap type followed by a VarInt-prefixed long array.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heightmap {
    pub kind: crate::MCVarInt,
    pub data: MCPrefixedArray<crate::MCLong, 256>,
}

impl MCType for Heightmap {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.kind.pack()?;
        bytes.extend(self.data.pack()?);
        Ok(bytes)
    }
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            kind: crate::MCVarInt::unpack(source)?,
            data: MCPrefixedArray::unpack(source)?,
        })
    }
}

/// Packed local X/Z, world Y, registry type and optional network NBT.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntity {
    pub packed_xz: crate::MCUnsignedByte,
    pub y: crate::MCShort,
    pub kind: crate::MCVarInt,
    pub data: Option<crate::nbt::Nbt>,
}

impl MCType for BlockEntity {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.packed_xz.pack()?;
        bytes.extend(self.y.pack()?);
        bytes.extend(self.kind.pack()?);
        if let Some(data) = &self.data {
            bytes.extend(data.encode_network()?);
        } else {
            bytes.push(0);
        }
        Ok(bytes)
    }
    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let packed_xz = crate::MCUnsignedByte::unpack(source)?;
        let y = crate::MCShort::unpack(source)?;
        let kind = crate::MCVarInt::unpack(source)?;
        let tag = crate::MCUnsignedByte::unpack(source)?.0;
        let data = if tag == 0 {
            None
        } else {
            Some(crate::nbt::Nbt::decode_network(
                &mut std::io::Cursor::new([tag]).chain(source),
            )?)
        };
        Ok(Self {
            packed_xz,
            y,
            kind,
            data,
        })
    }
}
