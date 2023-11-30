use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCDouble(f64);

impl From<MCDouble> for f64 {
    fn from(value: MCDouble) -> Self {
        value.0
    }
}

impl From<f64> for MCDouble {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for MCDouble {
    fn eq(&self, other: &f64) -> bool {
        &self.0 == other
    }
}

impl MCType for MCDouble {
    fn pack(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(f64::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
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
    fn test_double_pack() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_double_unpack() {
        unimplemented!()
    }
}
