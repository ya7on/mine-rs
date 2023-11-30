use crate::nbt::tags::base::{IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug)]
pub struct TagLong(i64);

impl IntoNBTTag for i64 {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagLong(self))
    }
}

impl NBTTag for TagLong {
    fn ty_id(&self) -> u8 {
        0x04
    }

    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(i64::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ]))
    }
}
