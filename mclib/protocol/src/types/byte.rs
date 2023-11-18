use crate::types::base::MCType;

#[derive(Debug)]
pub struct MCByte(i8);

impl From<i8> for MCByte {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl Into<i8> for MCByte {
    fn into(self) -> i8 {
        self.0
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

    fn unpack(src: &mut Vec<u8>) -> Self {
        Self(Self::read_byte(src) as i8)
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
        assert_eq!(MCByte::unpack(&mut vec![0b1000_0000]), -128);
        assert_eq!(MCByte::unpack(&mut vec![0b0111_1111]), 127);
        assert_eq!(MCByte::unpack(&mut vec![0b1111_1111]), -1);
    }
}
