use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

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
        result.extend((1000i32).to_be_bytes());
        for b in &self.0 {
            result.push(*b as u8);
        }
        result
    }
}
