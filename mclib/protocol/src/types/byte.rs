use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCByte(i8);

impl From<i8> for MCByte {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<MCByte> for i8 {
    fn from(value: MCByte) -> Self {
        value.0
    }
}

impl PartialEq<i8> for MCByte {
    fn eq(&self, other: &i8) -> bool {
        &self.0 == other
    }
}

impl MCType for MCByte {
    fn pack(&self) -> Vec<u8> {
        vec![self.0 as u8]
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(src.read_byte() as i8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_pack() {
        assert_eq!(vec![0b1000_0000], MCByte::from(-128).pack());
        assert_eq!(vec![0b0111_1111], MCByte::from(127).pack());
        assert_eq!(vec![0b1111_1111], MCByte::from(-1).pack());
    }

    #[test]
    fn test_byte_unpack() {
        assert_eq!(
            MCByte::unpack(&mut std::io::Cursor::new(vec![0b1000_0000])),
            -128
        );
        assert_eq!(
            MCByte::unpack(&mut std::io::Cursor::new(vec![0b0111_1111])),
            127
        );
        assert_eq!(
            MCByte::unpack(&mut std::io::Cursor::new(vec![0b1111_1111])),
            -1
        );
    }
}
