use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

#[derive(Debug)]
pub struct TagList(Vec<Box<dyn NBTTag>>);

impl IntoNBTTag for Vec<Box<dyn NBTTag>> {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        Box::new(TagList(self))
    }
}

impl NBTTag for TagList {
    fn ty_id(&self) -> u8 {
        0x09
    }

    fn pack(&self) -> Vec<u8> {
        let mut ty = 0x00;
        let len = self.0.len() as i32;
        let mut elements = Vec::new();

        for element in self.0.iter() {
            ty = element.ty_id();
            elements.extend(element.pack());
        }
        let mut result = Vec::new();
        result.push(ty);
        result.extend(len.to_be_bytes());
        result.extend(elements);
        result
    }
}
