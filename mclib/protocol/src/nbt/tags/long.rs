use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

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
}
