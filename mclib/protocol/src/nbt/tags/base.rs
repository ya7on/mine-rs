use crate::nbt::tags::byte::TagByte;
use crate::nbt::tags::byte_array::TagByteArray;
use crate::nbt::tags::compound::TagCompound;
use crate::nbt::tags::double::TagDouble;
use crate::nbt::tags::float::TagFloat;
use crate::nbt::tags::int::TagInt;
use crate::nbt::tags::list::TagList;
use crate::nbt::tags::long::TagLong;
use crate::nbt::tags::long_array::TagLongArray;
use crate::nbt::tags::short::TagShort;
use crate::nbt::tags::string::TagString;
use std::fmt::Debug;
use std::io::Read;

pub trait NBTTag: Debug {
    fn ty_id(&self) -> u8;
    fn pack(&self) -> Vec<u8>;
    fn unpack(src: &mut dyn Read) -> Self
    where
        Self: Sized;
}

pub trait IntoNBTTag {
    fn to_nbt(self) -> Box<dyn NBTTag>;
}

pub fn unpack_by_ty_id(ty_id: u8, src: &mut dyn Read) -> Box<dyn NBTTag> {
    match ty_id {
        1 => Box::new(TagByte::unpack(src)),
        2 => Box::new(TagShort::unpack(src)),
        3 => Box::new(TagInt::unpack(src)),
        4 => Box::new(TagLong::unpack(src)),
        5 => Box::new(TagFloat::unpack(src)),
        6 => Box::new(TagDouble::unpack(src)),
        7 => Box::new(TagByteArray::unpack(src)),
        8 => Box::new(TagString::unpack(src)),
        9 => Box::new(TagList::unpack(src)),
        10 => Box::new(TagCompound::unpack(src)),
        11 => {
            todo!()
        }
        12 => Box::new(TagLongArray::unpack(src)),
        _ => {
            todo!()
        }
    }
}
