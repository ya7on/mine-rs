use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCUShort(u16);

impl From<u16> for MCUShort {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<MCUShort> for u16 {
    fn from(value: MCUShort) -> Self {
        value.0
    }
}

impl PartialEq<u16> for MCUShort {
    fn eq(&self, other: &u16) -> bool {
        &self.0 == other
    }
}

impl MCType for MCUShort {
    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(u16::from_be_bytes([src.read_byte(), src.read_byte()]))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_ushort_pack() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_ushort_unpack() {
        unimplemented!()
    }
}
