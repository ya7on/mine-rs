use crate::nbt::tags::base::{IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug)]
pub struct TagByteArray(Vec<i8>);

impl IntoNBTTag for Vec<i8> {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagByteArray(self))
    }
}

impl NBTTag for TagByteArray {
    fn ty_id(&self) -> u8 {
        0x07
    }

    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend((self.0.len() as i32).to_be_bytes());
        for b in &self.0 {
            result.push(*b as u8);
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
            result.push(src.read_byte() as i8);
        }
        Self(result)
    }
}
