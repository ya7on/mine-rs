use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

const SEGMENT_BITS: u8 = 0b0111_1111;
const CONTINUE_BIT: u8 = 0b1000_0000;

#[derive(Debug, Clone)]
pub struct MCVarInt(i32);

impl From<i32> for MCVarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<MCVarInt> for i32 {
    fn from(value: MCVarInt) -> Self {
        value.0
    }
}

impl PartialEq<i32> for MCVarInt {
    fn eq(&self, other: &i32) -> bool {
        &self.0 == other
    }
}

impl MCType for MCVarInt {
    fn pack(&self) -> Vec<u8> {
        let mut value = self.0;
        let mut result = Vec::new();

        for _ in 0..100 {
            if (value & !(SEGMENT_BITS as i32)) == 0 {
                result.push(value as u8);
                break;
            }

            result.push(((value as u8) & SEGMENT_BITS) | CONTINUE_BIT);

            value >>= 7;
            value &= i32::MAX >> 6;
        }

        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let mut value = 0i32;

        for i in 0..5 {
            let current_byte = src.read_byte();
            value |= ((current_byte & SEGMENT_BITS) as i32) << (i * 7);

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }
        }

        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_pack() {
        assert_eq!(vec![0x00], MCVarInt::from(0).pack());
        assert_eq!(vec![0x01], MCVarInt::from(1).pack());
        assert_eq!(vec![0x02], MCVarInt::from(2).pack());
        assert_eq!(vec![0x7f], MCVarInt::from(127).pack());
        assert_eq!(vec![0x80, 0x01], MCVarInt::from(128).pack());
        assert_eq!(vec![0xff, 0x01], MCVarInt::from(255).pack());
        assert_eq!(vec![0xdd, 0xc7, 0x01], MCVarInt::from(25565).pack());
        assert_eq!(vec![0xff, 0xff, 0x7f], MCVarInt::from(2097151).pack());
        assert_eq!(
            vec![0xff, 0xff, 0xff, 0xff, 0x07],
            MCVarInt::from(2147483647).pack()
        );
        assert_eq!(
            vec![0xff, 0xff, 0xff, 0xff, 0x0f],
            MCVarInt::from(-1).pack()
        );
        assert_eq!(
            vec![0x80, 0x80, 0x80, 0x80, 0x08],
            MCVarInt::from(-2147483648).pack()
        );
    }

    #[test]
    fn test_varint_unpack() {
        assert_eq!(MCVarInt::unpack(&mut std::io::Cursor::new(vec![0x00])), 0);
        assert_eq!(MCVarInt::unpack(&mut std::io::Cursor::new(vec![0x01])), 1);
        assert_eq!(MCVarInt::unpack(&mut std::io::Cursor::new(vec![0x02])), 2);
        assert_eq!(MCVarInt::unpack(&mut std::io::Cursor::new(vec![0x7f])), 127);
        assert_eq!(
            MCVarInt::unpack(&mut std::io::Cursor::new(vec![0x80, 0x01])),
            128
        );
        assert_eq!(
            MCVarInt::unpack(&mut std::io::Cursor::new(vec![0xff, 0x01])),
            255
        );
        assert_eq!(
            MCVarInt::unpack(&mut std::io::Cursor::new(vec![0xdd, 0xc7, 0x01])),
            25565
        );
        assert_eq!(
            MCVarInt::unpack(&mut std::io::Cursor::new(vec![0xff, 0xff, 0x7f])),
            2097151
        );
        assert_eq!(
            MCVarInt::unpack(&mut std::io::Cursor::new(vec![
                0xff, 0xff, 0xff, 0xff, 0x07
            ])),
            2147483647
        );
        assert_eq!(
            MCVarInt::unpack(&mut std::io::Cursor::new(vec![
                0xff, 0xff, 0xff, 0xff, 0x0f
            ])),
            -1
        );
        assert_eq!(
            MCVarInt::unpack(&mut std::io::Cursor::new(vec![
                0x80, 0x80, 0x80, 0x80, 0x08
            ])),
            -2147483648
        );
    }
}
