use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCLong(i64);

impl From<MCLong> for i64 {
    fn from(value: MCLong) -> Self {
        value.0
    }
}

impl From<i64> for MCLong {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl PartialEq<i64> for MCLong {
    fn eq(&self, other: &i64) -> bool {
        &self.0 == other
    }
}

impl MCType for MCLong {
    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(i64::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
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
    fn test_long_pack() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_long_unpack() {
        unimplemented!()
    }
}
