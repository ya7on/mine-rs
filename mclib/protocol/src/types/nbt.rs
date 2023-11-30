use crate::nbt::NBT;
use crate::types::base::MCType;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCNBT(NBT);

impl From<NBT> for MCNBT {
    fn from(value: NBT) -> Self {
        Self(value)
    }
}

impl From<MCNBT> for NBT {
    fn from(value: MCNBT) -> Self {
        value.0
    }
}

impl MCType for MCNBT {
    fn pack(&self) -> Vec<u8> {
        self.0.pack()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(NBT::unpack(src, false))
    }
}
