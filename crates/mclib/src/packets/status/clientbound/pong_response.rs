use std::io::Read;

use crate::{MCLong, MCType, ProtocolError};

/// The server's response to a status ping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PongResponse {
    pub timestamp: MCLong,
}

impl MCType for PongResponse {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        self.timestamp.pack()
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            timestamp: MCLong::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_only_the_big_endian_long_body() {
        let packet = PongResponse {
            timestamp: 0x0102_0304_0506_0708_i64.into(),
        };

        assert_eq!(packet.pack().unwrap(), [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
