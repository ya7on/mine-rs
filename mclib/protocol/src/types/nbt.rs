use crate::nbt::NBT;
use crate::types::base::MCType;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCNBT(NBT);

impl MCType for MCNBT {
    fn pack(&self) -> Vec<u8> {
        self.0.pack()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(NBT::unpack(src, false))
    }
}
