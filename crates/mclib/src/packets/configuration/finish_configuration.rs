use std::io::Read;

use crate::{MCType, ProtocolError};

/// Empty Finish Configuration body, identical in both directions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FinishConfiguration;

impl MCType for FinishConfiguration {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::new())
    }

    fn unpack(_source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self)
    }
}
