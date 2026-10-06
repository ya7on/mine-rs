use std::io::Read;

use crate::{MCType, ProtocolError};

/// Acknowledges Login Success before the connection enters Configuration.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LoginAcknowledged;

impl MCType for LoginAcknowledged {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::new())
    }

    fn unpack(_source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PacketFrame;

    #[test]
    fn matches_the_known_uncompressed_frame() {
        let body = LoginAcknowledged.pack().unwrap();
        assert!(body.is_empty());
        assert_eq!(PacketFrame::new(3, body).pack().unwrap(), [1, 3]);
        assert_eq!(
            LoginAcknowledged::unpack(&mut [].as_slice()).unwrap(),
            LoginAcknowledged
        );
    }
}
