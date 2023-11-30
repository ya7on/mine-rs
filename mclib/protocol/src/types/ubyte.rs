use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCUByte(u8);

impl From<u8> for MCUByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MCUByte> for u8 {
    fn from(value: MCUByte) -> Self {
        value.0
    }
}

impl PartialEq<u8> for MCUByte {
    fn eq(&self, other: &u8) -> bool {
        &self.0 == other
    }
}

impl MCType for MCUByte {
    fn pack(&self) -> Vec<u8> {
        vec![self.0]
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(src.read_byte())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ubyte_pack() {
        assert_eq!(vec![0b1000_0000], MCUByte::from(128).pack());
        assert_eq!(vec![0b0111_1111], MCUByte::from(127).pack());
        assert_eq!(vec![0b1111_1111], MCUByte::from(255).pack());
    }

    #[test]
    fn test_ubyte_unpack() {
        assert_eq!(
            MCUByte::unpack(&mut std::io::Cursor::new(vec![0b1000_0000])),
            128
        );
        assert_eq!(
            MCUByte::unpack(&mut std::io::Cursor::new(vec![0b0111_1111])),
            127
        );
        assert_eq!(
            MCUByte::unpack(&mut std::io::Cursor::new(vec![0b1111_1111])),
            255
        );
    }
}
