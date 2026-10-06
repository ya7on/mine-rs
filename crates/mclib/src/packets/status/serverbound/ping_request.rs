use std::io::Read;

use crate::{MCLong, MCType, ProtocolError};

/// Carries the timestamp used to measure status-ping latency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PingRequest {
    pub timestamp: MCLong,
}

impl MCType for PingRequest {
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
        let packet = PingRequest {
            timestamp: (-2_i64).into(),
        };

        assert_eq!(
            packet.pack().unwrap(),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe]
        );
    }
}
