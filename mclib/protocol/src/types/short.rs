use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCShort(i16);

impl From<i16> for MCShort {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<MCShort> for i16 {
    fn from(value: MCShort) -> Self {
        value.0
    }
}

impl PartialEq<i16> for MCShort {
    fn eq(&self, other: &i16) -> bool {
        &self.0 == other
    }
}

impl MCType for MCShort {
    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(i16::from_be_bytes([src.read_byte(), src.read_byte()]))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_short_pack() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_short_unpack() {
        unimplemented!()
    }
}
