use crate::nbt::tags::base::{IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug)]
pub struct TagFloat(f32);

impl IntoNBTTag for f32 {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagFloat(self))
    }
}

impl NBTTag for TagFloat {
    fn ty_id(&self) -> u8 {
        0x05
    }

    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(f32::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ]))
    }
}
