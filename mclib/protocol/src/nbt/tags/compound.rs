use crate::nbt::tags::base::{unpack_by_ty_id, IntoNBTTag, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

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

    fn unpack(src: &mut dyn Read) -> Self {
        let mut result = Vec::<(String, Box<dyn NBTTag>)>::new();
        let read_name = |src: &mut dyn Read| {
            let name_len = u16::from_be_bytes([src.read_byte(), src.read_byte()]);
            let mut name_bytes = Vec::new();
            for _ in 0..name_len {
                name_bytes.push(src.read_byte());
            }
            String::from_utf8(name_bytes).unwrap()
        };
        loop {
            let ty_id = src.read_byte();
            if ty_id == 0 {
                break;
            }
            result.push((read_name(src), unpack_by_ty_id(ty_id, src)));
        }
        Self(result)
    }
}
