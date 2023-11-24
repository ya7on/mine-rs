use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

#[derive(Debug)]
pub struct TagShort(i16);

impl IntoNBTTag for i16 {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagShort(self))
    }
}

impl NBTTag for TagShort {
    fn ty_id(&self) -> u8 {
        0x02
    }

    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}
