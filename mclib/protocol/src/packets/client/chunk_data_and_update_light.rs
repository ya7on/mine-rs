use crate::chunk_format::ChunkData;
use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::bitset::MCBitSet;
use crate::types::byte::MCByte;
use crate::types::byte_array::MCByteArray;
use crate::types::int::MCInt;
use crate::types::nbt::MCNBT;
use crate::types::short::MCShort;
use crate::types::varint::MCVarInt;
use mclib_macros::{MCPacket, MCType};
use std::io::Read;

#[derive(MCType, Debug, Clone)]
pub struct BlockEntity {
    pub packed_xy: MCByte,
    pub y: MCShort,
    pub ty: MCVarInt,
    pub data: MCNBT,
}

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x25)]
pub struct ChunkDataAndUpdateLight {
    pub chunk_x: MCInt,
    pub chunk_z: MCInt,
    pub heightmaps: MCNBT,
    pub data: MCByteArray<ChunkData>,
    pub block_entities: Vec<BlockEntity>,
    pub sky_light_mask: MCBitSet,
    pub block_light_mask: MCBitSet,
    pub empty_sky_light_mask: MCBitSet,
    pub empty_block_light_mask: MCBitSet,
    pub sky_lights: Vec<Vec<MCByte>>,
    pub block_lights: Vec<Vec<MCByte>>,
}

#[cfg(test)]
mod tests {
    use crate::chunk_format::data_array::DataArray;
    use crate::chunk_format::palleted_container::PalletedContainer;
    use crate::chunk_format::section::ChunkSection;
    use crate::chunk_format::ChunkData;
    use crate::nbt::tags::base::IntoNBTTag;
    use crate::nbt::NBT;
    use crate::packets::base::MCPacket;
    use crate::packets::client::chunk_data_and_update_light::ChunkDataAndUpdateLight;
    use crate::types::bitset::MCBitSet;
    use crate::types::byte_array::MCByteArray;
    use crate::types::long::MCLong;

    #[test]
    fn test_chunk_data() {
        let reference =
            include_bytes!("../../../../../assets/tests/chunk_data_and_update_light.packet");

        let heightmap = NBT(
            None,
            vec![
                (
                    "MOTION_BLOCKING",
                    DataArray::from(vec![4; 256]).pack(9).to_nbt(),
                ),
                (
                    "WORLD_SURFACE",
                    DataArray::from(vec![4; 256]).pack(9).to_nbt(),
                ),
            ]
            .to_nbt(),
        );

        let mut data = Vec::new();
        data.extend(vec![1; 256]);
        data.extend(vec![2; 512]);
        data.extend(vec![3; 256]);
        data.extend(vec![0; 3072]);
        let mut chunks = Vec::new();
        chunks.push(ChunkSection {
            block_count: 1024.into(),
            block_states: PalletedContainer::Indirect {
                bits_per_entry: 4.into(),
                pallete: vec![0.into(), 79.into(), 10.into(), 9.into()],
                data: DataArray::from(data)
                    .pack(4)
                    .iter()
                    .map(|i| i.clone().into())
                    .collect::<Vec<MCLong>>(),
            },
            biomes: PalletedContainer::SingleValued(39.into()),
        });
        chunks.extend(vec![
            ChunkSection {
                block_count: 0.into(),
                block_states: PalletedContainer::SingleValued(0.into()),
                biomes: PalletedContainer::SingleValued(39.into()),
            };
            23
        ]);
        let mut sky_light_mask = MCBitSet::default();
        sky_light_mask.push(false);
        sky_light_mask.push(true);
        sky_light_mask.push(true);
        let block_light_mask = MCBitSet::default();
        let mut empty_sky_light_mask = MCBitSet::default();
        empty_sky_light_mask.push(true);
        let mut empty_block_light_mask = MCBitSet::default();
        empty_block_light_mask.push(true);
        empty_block_light_mask.push(true);
        empty_block_light_mask.push(true);

        let mut sky = vec![0.into(); 512];
        sky.extend(vec![(0x0F | (0x0F << 4)).into(); 1536]);
        let sky_lights = vec![sky, vec![(0x0F | (0x0F << 4)).into(); 2048]];

        let block_lights = vec![];

        let chunk_data_and_update_light = ChunkDataAndUpdateLight {
            chunk_x: (-1 as i32).into(),
            chunk_z: 0.into(),
            heightmaps: heightmap.into(),
            data: MCByteArray::new(ChunkData(chunks)),
            block_entities: vec![],
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_lights,
            block_lights,
        };

        let packed = &chunk_data_and_update_light.pack()[2..];
        assert_eq!(packed, reference);
    }
}
