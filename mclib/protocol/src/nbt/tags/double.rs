use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

#[derive(Debug)]
pub struct TagDouble(f64);

impl IntoNBTTag for f64 {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagDouble(self))
    }
}

impl NBTTag for TagDouble {
    fn ty_id(&self) -> u8 {
        0x06
    }

    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}
