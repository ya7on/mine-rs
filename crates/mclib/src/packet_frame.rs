//! Uncompressed Minecraft packet framing.

use std::io::{Cursor, Read, Write};

use crate::types::{MCType, MCVarInt, ProtocolError};

/// Maximum value of the three-byte packet-length VarInt.
pub const MAX_PACKET_LENGTH: usize = (1 << 21) - 1;

/// A packet ID and its encoded body, without compression framing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketFrame {
    pub packet_id: MCVarInt,
    pub body: Vec<u8>,
}

impl PacketFrame {
    pub fn new(packet_id: i32, body: Vec<u8>) -> Result<Self, ProtocolError> {
        if packet_id < 0 {
            return Err(ProtocolError::InvalidPacketId(packet_id));
        }

        let frame = Self {
            packet_id: packet_id.into(),
            body,
        };
        frame.packet_length()?;
        Ok(frame)
    }

    /// Reads one complete uncompressed frame from a stream.
    pub fn read(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let packet_length = read_packet_length(source)?;
        if packet_length <= 0 {
            return Err(ProtocolError::InvalidPacketLength(packet_length));
        }

        let packet_length = packet_length as usize;
        let mut payload = vec![0_u8; packet_length];
        source.read_exact(&mut payload)?;

        let mut payload = Cursor::new(payload);
        let packet_id = MCVarInt::unpack(&mut payload)?;
        if packet_id.0 < 0 {
            return Err(ProtocolError::InvalidPacketId(packet_id.0));
        }

        let body_start = payload.position() as usize;
        let body = payload.into_inner().split_off(body_start);
        Ok(Self { packet_id, body })
    }

    /// Writes one complete uncompressed frame to a stream.
    pub fn write(&self, destination: &mut dyn Write) -> Result<(), ProtocolError> {
        if self.packet_id.0 < 0 {
            return Err(ProtocolError::InvalidPacketId(self.packet_id.0));
        }

        let packet_id = self.packet_id.pack()?;
        let packet_length = self.packet_length()?;
        destination.write_all(&MCVarInt(packet_length as i32).pack()?)?;
        destination.write_all(&packet_id)?;
        destination.write_all(&self.body)?;
        Ok(())
    }

    /// Encodes one complete uncompressed frame into a byte vector.
    pub fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let packet_length = self.packet_length()?;
        let mut encoded = Vec::with_capacity(packet_length + 3);
        self.write(&mut encoded)?;
        Ok(encoded)
    }

    fn packet_length(&self) -> Result<usize, ProtocolError> {
        let packet_id_length = self.packet_id.pack()?.len();
        let length =
            packet_id_length
                .checked_add(self.body.len())
                .ok_or(ProtocolError::PacketTooLarge {
                    length: usize::MAX,
                    max_length: MAX_PACKET_LENGTH,
                })?;

        if length > MAX_PACKET_LENGTH {
            return Err(ProtocolError::PacketTooLarge {
                length,
                max_length: MAX_PACKET_LENGTH,
            });
        }

        Ok(length)
    }
}

fn read_packet_length(source: &mut dyn Read) -> Result<i32, ProtocolError> {
    let mut result = 0_u32;

    for index in 0..3 {
        let mut byte = [0_u8];
        source.read_exact(&mut byte)?;
        result |= u32::from(byte[0] & 0x7f) << (7 * index);
        if byte[0] & 0x80 == 0 {
            return Ok(result as i32);
        }
    }

    Err(ProtocolError::PacketLengthVarIntTooLong)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_status_request_has_length_and_id_only() {
        let frame = PacketFrame::new(0, Vec::new()).unwrap();

        assert_eq!(frame.pack().unwrap(), [0x01, 0x00]);
    }

    #[test]
    fn reads_and_writes_a_complete_frame() {
        let frame = PacketFrame::new(1, vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        let encoded = frame.pack().unwrap();

        assert_eq!(encoded, [0x09, 0x01, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(PacketFrame::read(&mut Cursor::new(encoded)).unwrap(), frame);
    }

    #[test]
    fn uses_a_multibyte_length_var_int() {
        let frame = PacketFrame::new(0, vec![0; 127]).unwrap();
        let encoded = frame.pack().unwrap();

        assert_eq!(&encoded[..3], &[0x80, 0x01, 0x00]);
    }

    #[test]
    fn rejects_zero_length_frames() {
        let error = PacketFrame::read(&mut Cursor::new([0x00])).unwrap_err();

        assert!(matches!(error, ProtocolError::InvalidPacketLength(0)));
    }

    #[test]
    fn rejects_packet_length_var_ints_over_three_bytes() {
        let error = PacketFrame::read(&mut Cursor::new([0x80, 0x80, 0x80])).unwrap_err();

        assert!(matches!(error, ProtocolError::PacketLengthVarIntTooLong));
    }
}
