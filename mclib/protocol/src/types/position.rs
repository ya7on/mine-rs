use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCPosition {
    pub x: i32,
    pub z: i32,
    pub y: i16,
}

impl PartialEq for MCPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z && self.y == other.y
    }
}

impl MCType for MCPosition {
    fn pack(&self) -> Vec<u8> {
        let result = ((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF);
        result.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let data = i64::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ]);

        let x = (data >> 38) as i32;
        let y = (data << 52 >> 52) as i16;
        let z = (data << 26 >> 38) as i32;

        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::base::MCType;
    use crate::types::position::MCPosition;

    #[test]
    fn test_position_pack() {
        let result = 0b01000110000001110110001100_10110000010101101101001000_001100111111i64;
        let position = MCPosition {
            x: 18357644,
            z: -20882616,
            y: 831,
        };

        assert_eq!(result.to_be_bytes().to_vec(), position.pack());
    }

    #[test]
    fn test_position_unpack() {
        let result = 0b01000110000001110110001100_10110000010101101101001000_001100111111i64;
        let position = MCPosition::unpack(&mut std::io::Cursor::new(result.to_be_bytes()));

        assert_eq!(
            position,
            MCPosition {
                x: 18357644,
                z: -20882616,
                y: 831,
            }
        );
    }
}
