use crate::nbt::tags::base::{IntoNBTTag, NBTTag};

#[derive(Debug)]
pub struct TagCompound(Vec<(String, Box<dyn NBTTag>)>);

impl IntoNBTTag for Vec<(&str, Box<dyn NBTTag>)> {
    fn to_nbt(self) -> Box<(dyn NBTTag)> {
        let mut result = Vec::new();
        for (key, value) in self {
            result.push((key.to_owned(), value))
        }
        Box::new(TagCompound(result))
    }
}

impl NBTTag for TagCompound {
    fn ty_id(&self) -> u8 {
        0x0a
    }

    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for (key, element) in self.0.iter() {
            result.push(element.ty_id());
            let element_name = key.as_bytes();
            result.extend((element_name.len() as u16).to_be_bytes());
            result.extend(element_name);
            result.extend(element.pack());
        }
        result.push(0x00);
        result
    }
}
