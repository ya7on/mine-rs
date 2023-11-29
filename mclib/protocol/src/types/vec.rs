use crate::types::base::MCType;
use crate::types::varint::MCVarInt;
use std::io::Read;

impl<P: MCType> MCType for Vec<P> {
    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(MCVarInt::from(self.len() as i32).pack());
        for item in self.iter() {
            result.extend(item.pack());
        }
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let mut result = Vec::new();
        let length = MCVarInt::unpack(src);
        for _ in 0..length.into() {
            result.push(P::unpack(src));
        }
        result
    }
}
