use std::io::Read;

use crate::{MCType, ProtocolError};

/// Requests the server's status document.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatusRequest;

impl MCType for StatusRequest {
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

    #[test]
    fn body_is_empty() {
        assert_eq!(StatusRequest.pack().unwrap(), Vec::<u8>::new());
    }
}
