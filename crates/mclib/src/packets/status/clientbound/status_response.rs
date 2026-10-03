use std::io::Read;

use crate::types::{MCString, MCType, ProtocolError};

/// The server's JSON status document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusResponse {
    pub json_response: MCString,
}

impl MCType for StatusResponse {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        self.json_response.pack()
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            json_response: MCString::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn packs_only_the_var_int_prefixed_json_body() {
        let packet = StatusResponse {
            json_response: r#"{"version":{"protocol":777}}"#.into(),
        };

        let encoded = packet.pack().unwrap();
        assert_eq!(encoded[0] as usize, encoded.len() - 1);
        assert_eq!(
            StatusResponse::unpack(&mut Cursor::new(encoded)).unwrap(),
            packet
        );
    }
}
