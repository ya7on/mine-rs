use crate::nbt::tags::base::{unpack_by_ty_id, IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

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

    fn unpack(src: &mut dyn Read) -> Self {
        let mut result = Vec::<Box<dyn NBTTag>>::new();
        let ty_id = src.read_byte();
        let length = i32::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ]);
        for _ in 0..length {
            result.push(unpack_by_ty_id(ty_id, src));
        }
        Self(result)
    }
}
