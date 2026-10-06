use std::io::Read;

use crate::{MCType, ProtocolError};

/// A packed block `Position`: x (26 bits), z (26 bits), y (12 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MCPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl MCPosition {
    /// Largest absolute coordinate on the x and z axes.
    const HORIZONTAL_MAX: i32 = 33_554_431;

    /// Largest absolute coordinate on the y axis.
    const VERTICAL_MAX: i32 = 2_047;

    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl MCType for MCPosition {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        for (value, max) in [
            (self.x, Self::HORIZONTAL_MAX),
            (self.z, Self::HORIZONTAL_MAX),
            (self.y, Self::VERTICAL_MAX),
        ] {
            if value > max || value < -max - 1 {
                return Err(ProtocolError::Overflow);
            }
        }

        let x = i64::from(self.x) & 0x3ff_ffff;
        let y = i64::from(self.y) & 0xfff;
        let z = i64::from(self.z) & 0x3ff_ffff;
        let packed = (x << 38) | (z << 12) | y;

        Ok(packed.to_be_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 8];
        source.read_exact(&mut bytes)?;
        let packed = i64::from_be_bytes(bytes);

        // Arithmetic shifts restore the sign of each part.
        let x = (packed >> 38) as i32;
        let y = ((packed << 52) >> 52) as i32;
        let z = ((packed << 26) >> 38) as i32;
        Ok(Self { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrips_sample_position() {
        let value = MCPosition::new(18_357_644, 831, -20_882_616);

        let encoded = value.pack().unwrap();

        assert_eq!(
            MCPosition::unpack(&mut Cursor::new(encoded)).unwrap(),
            value
        );
    }

    #[test]
    fn rejects_out_of_range_coordinates() {
        for value in [
            MCPosition::new(33_554_432, 0, 0),
            MCPosition::new(0, 0, 33_554_432),
            MCPosition::new(0, 2_048, 0),
            MCPosition::new(-33_554_433, 0, 0),
            MCPosition::new(0, -2_049, 0),
        ] {
            let error = value.pack().unwrap_err();

            assert!(matches!(error, ProtocolError::Overflow));
        }
    }

    #[test]
    fn roundtrips_boundary_coordinates() {
        // x and z span ±33,554,432 and y spans ±2048 on the wire.
        for value in [
            MCPosition::new(0, 0, 0),
            MCPosition::new(33_554_431, 2047, 33_554_431),
            MCPosition::new(-33_554_432, -2048, -33_554_432),
            MCPosition::new(33_554_431, 2047, -33_554_432),
        ] {
            let encoded = value.pack().unwrap();
            assert_eq!(
                MCPosition::unpack(&mut Cursor::new(encoded)).unwrap(),
                value
            );
        }
    }
}
