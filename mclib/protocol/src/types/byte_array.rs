use crate::types::base::MCType;
use crate::types::varint::MCVarInt;
use crate::utils::TcpUtils;
use std::fmt::Debug;
use std::io::{Cursor, Read};

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
        let length = MCVarInt::unpack(src);
        let mut buffer = Vec::new();
        for _ in 0..length.into() {
            buffer.push(src.read_byte());
        }
        let mut cursor = Cursor::new(buffer);
        let inner = T::unpack(&mut cursor);
        Self(inner)
    }
}
