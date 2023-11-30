use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCInt(i32);

impl From<MCInt> for i32 {
    fn from(value: MCInt) -> Self {
        value.0
    }
}

impl From<i32> for MCInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl PartialEq<i32> for MCInt {
    fn eq(&self, other: &i32) -> bool {
        &self.0 == other
    }
}

impl MCType for MCInt {
    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(i32::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ]))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_int_pack() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_int_unpack() {
        unimplemented!()
    }
}
