use crate::chunk_format::section::ChunkSection;
use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

pub mod data_array;
pub mod palleted_container;
pub mod section;

#[derive(Debug, Clone)]
pub struct ChunkData(pub Vec<ChunkSection>);

impl MCType for ChunkData {
    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for section in self.0.iter() {
            result.extend(section.pack());
        }
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let mut result = Vec::new();
        // todo use error handling
        let world_height = 384;
        for _ in 0..world_height / 16 - 1 {
            result.push(ChunkSection::unpack(src));
        }
        src.read_byte();

        Self(result)
    }
}
