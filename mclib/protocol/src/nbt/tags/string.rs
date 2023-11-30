use crate::nbt::tags::base::{IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug)]
pub struct TagString(Vec<u8>);

impl IntoNBTTag for &str {
    fn to_nbt(self) -> Box<(dyn NBTTag)> {
        Box::new(TagString(self.as_bytes().to_vec()))
    }
}

impl IntoNBTTag for &[u8] {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagString(self.to_vec()))
    }
}

impl NBTTag for TagString {
    fn ty_id(&self) -> u8 {
        0x08
    }

    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend((self.0.len() as u16).to_be_bytes());
        result.extend(&self.0);
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let length = u16::from_be_bytes([src.read_byte(), src.read_byte()]);
        let mut result = Vec::new();
        for _ in 0..length {
            result.push(src.read_byte());
        }
        Self(result)
    }
}
