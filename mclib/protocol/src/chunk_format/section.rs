use crate::chunk_format::palleted_container::PalletedContainer;
use crate::types::base::MCType;
use crate::types::short::MCShort;
use mclib_macros::MCType;
use std::io::Read;

#[derive(MCType, Debug, Clone)]
pub struct ChunkSection {
    pub block_count: MCShort,
    pub block_states: PalletedContainer<8>,
    pub biomes: PalletedContainer<3>,
}

impl ChunkSection {}

#[cfg(test)]
mod tests {
    use crate::chunk_format::palleted_container::PalletedContainer;
    use crate::chunk_format::section::ChunkSection;
    use crate::types::base::MCType;

    #[test]
    fn test_chunk_single_valued_pallete() {
        let reference = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x27, 0x03, 0x01, 0xCC, 0xFF, 0xCC, 0xFF,
            0xCC, 0xFF, 0xCC, 0xFF,
        ];

        let chunk = ChunkSection {
            block_count: 0.into(),
            block_states: PalletedContainer::SingleValued(0.into()),
            biomes: PalletedContainer::Indirect {
                bits_per_entry: 1.into(),
                pallete: vec![39.into(), 3.into()],
                data: vec![
                    i64::from_be_bytes([0xCC, 0xFF, 0xCC, 0xFF, 0xCC, 0xFF, 0xCC, 0xFF]).into(),
                ],
            },
        };

        assert_eq!(chunk.pack(), reference)
    }
}
