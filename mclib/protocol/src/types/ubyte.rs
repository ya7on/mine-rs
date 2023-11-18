use crate::types::base::MCType;

#[derive(Debug)]
pub struct MCUByte(u8);

impl From<u8> for MCUByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for MCUByte {
    fn into(self) -> u8 {
        self.0
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

    fn unpack(src: &mut Vec<u8>) -> Self {
        Self(src.remove(0))
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
        assert_eq!(MCUByte::unpack(&mut vec![0b1000_0000]), 128);
        assert_eq!(MCUByte::unpack(&mut vec![0b0111_1111]), 127);
        assert_eq!(MCUByte::unpack(&mut vec![0b1111_1111]), 255);
    }
}