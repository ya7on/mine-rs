use crate::nbt::tags::base::{IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug)]
pub struct TagLongArray(Vec<i64>);

impl IntoNBTTag for Vec<i64> {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagLongArray(self))
    }
}

impl NBTTag for TagLongArray {
    fn ty_id(&self) -> u8 {
        0x0C
    }

    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend((self.0.len() as i32).to_be_bytes());
        for b in &self.0 {
            result.extend(b.to_be_bytes());
        }
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let length = i32::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ]);
        let mut result = Vec::new();
        for _ in 0..length {
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
