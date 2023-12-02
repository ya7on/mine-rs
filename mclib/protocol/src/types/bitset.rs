use crate::types::base::MCType;
use crate::types::varint::MCVarInt;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone, Default)]
pub struct MCBitSet(Vec<i64>);

impl MCType for MCBitSet {
    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(MCVarInt::from(self.0.len() as i32).pack());
        for long in self.0.iter() {
            result.extend(long.to_be_bytes());
        }
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let len = MCVarInt::unpack(src);
        let mut result = Vec::new();
        for _ in 0..len.into() {
            result.push(i64::from_be_bytes([
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
            ]));
        }
        Self(result)
    }
}
