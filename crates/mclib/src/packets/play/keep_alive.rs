use std::io::Read;

use crate::{MCLong, MCType, ProtocolError};

/// The client echoes the server's ID unchanged in a Play Keep Alive reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepAlive {
    pub id: MCLong,
}

impl MCType for KeepAlive {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        self.id.pack()
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            id: MCLong::unpack(source)?,
        })
    }
}
