use std::fmt::Debug;

pub trait NBTTag: Debug {
    fn ty_id(&self) -> u8;
    fn pack(&self) -> Vec<u8>;
}

pub trait IntoNBTTag {
    fn to_nbt(self) -> Box<dyn NBTTag>;
}
