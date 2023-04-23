use crate::types::{MinecraftType, CONTINUE_BIT, SEGMENT_BITS};
use crate::MinecraftUnsignedByte;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;

#[derive(Debug)]
pub struct MinecraftVarInt(pub i32);

#[async_trait]
impl MinecraftType for MinecraftVarInt {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let mut value = 0;
        let mut i = 0;
        loop {
            let byte = MinecraftUnsignedByte::parse_from(io).await?.0;
            value |= ((byte & SEGMENT_BITS) as i32) << (7 * i);
            if byte & CONTINUE_BIT == 0 {
                break;
            }
            i += 1;
        }
        Ok(Self(value))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        let mut value = self.0;

        if value == 0 {
            return Ok(vec![0u8]);
        }

        let mut buf = [0];
        while value != 0 {
            buf[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i32::MAX >> 6);
            if value != 0 {
                buf[0] |= 0b1000_0000;
            }
            std::io::Write::write(&mut result, &buf).unwrap();
        }

        Ok(result)
    }
}
