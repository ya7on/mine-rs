use crate::types::base::MCType;
use crate::types::varint::MCVarInt;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone, Default)]
pub struct MCBitSet(Vec<bool>);

impl MCBitSet {
    pub fn push(&mut self, value: bool) {
        self.0.push(value)
    }
}

impl MCType for MCBitSet {
    fn pack(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for long in self.0.chunks(64) {
            let mut num = 0_i64;
            for (bit_index, bit) in long.iter().enumerate() {
                num |= (*bit as i64) << bit_index;
            }
            data.extend(num.to_be_bytes());
        }
        let mut result = Vec::new();
        result.extend(MCVarInt::from((data.len() / 8) as i32).pack());
        result.extend(data);
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let len = MCVarInt::unpack(src);
        let mut result = Vec::new();
        for _ in 0..len.into() {
            let mut num = i64::from_be_bytes([
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
                src.read_byte(),
            ]);

            for _ in 0..64 {
                result.push((num & 0b1) == 1);
                num >>= 1;
            }
        }
        Self(result)
    }
}
