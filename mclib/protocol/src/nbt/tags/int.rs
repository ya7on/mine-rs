use crate::nbt::tags::base::{IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug)]
pub struct TagInt(i32);

impl IntoNBTTag for i32 {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagInt(self))
    }
}

impl NBTTag for TagInt {
    fn ty_id(&self) -> u8 {
        0x03
    }

    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(i32::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ]))
    }
}
