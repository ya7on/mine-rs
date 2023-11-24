use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

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
}
