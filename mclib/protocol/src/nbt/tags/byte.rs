use crate::nbt::tags::base::{IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug)]
pub struct TagByte(i8);

impl IntoNBTTag for i8 {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagByte(self))
    }
}

impl NBTTag for TagByte {
    fn ty_id(&self) -> u8 {
        0x01
    }

    fn pack(&self) -> Vec<u8> {
        vec![self.0 as u8]
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(src.read_byte() as i8)
    }
}
