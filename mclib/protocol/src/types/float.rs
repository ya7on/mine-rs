use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCFloat(f32);

impl From<MCFloat> for f32 {
    fn from(value: MCFloat) -> Self {
        value.0
    }
}

impl From<f32> for MCFloat {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl PartialEq<f32> for MCFloat {
    fn eq(&self, other: &f32) -> bool {
        &self.0 == other
    }
}

impl MCType for MCFloat {
    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(f32::from_be_bytes([
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
    fn test_float_pack() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_float_unpack() {
        unimplemented!()
    }
}
