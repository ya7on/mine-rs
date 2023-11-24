use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

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
}
