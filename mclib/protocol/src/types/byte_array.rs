use crate::types::base::MCType;
use crate::types::varint::MCVarInt;
use std::fmt::Debug;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCByteArray<T: MCType + Debug + Clone>(T);

impl<T: MCType + Debug + Clone> MCByteArray<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl<T: MCType + Debug + Clone> MCType for MCByteArray<T> {
    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let body = self.0.pack();
        result.extend(MCVarInt::from(body.len() as i32).pack());
        result.extend(body);
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let _length = MCVarInt::unpack(src);
        let inner = T::unpack(src);
        Self(inner)
    }
}
