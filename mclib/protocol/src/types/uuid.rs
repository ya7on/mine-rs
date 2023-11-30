use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MCUuid(Uuid);

impl From<Uuid> for MCUuid {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<MCUuid> for Uuid {
    fn from(value: MCUuid) -> Self {
        value.0
    }
}

impl PartialEq<Uuid> for MCUuid {
    fn eq(&self, other: &Uuid) -> bool {
        &self.0 == other
    }
}

impl MCType for MCUuid {
    fn pack(&self) -> Vec<u8> {
        self.0.as_u128().to_be_bytes().to_vec()
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(Uuid::from_u128(u128::from_be_bytes([
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
            src.read_byte(),
        ])))
    }
}
