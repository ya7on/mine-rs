//! Uncompressed Minecraft packet framing.

use std::io::{Cursor, Read, Write};

use crate::{MCType, MCVarInt, ProtocolError};

/// Maximum value of the three-byte packet-length `VarInt`.
pub const MAX_PACKET_LENGTH: usize = (1 << 21) - 1;

/// A packet ID and its encoded body, without compression framing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketFrame {
    pub packet_id: MCVarInt,
    pub body: Vec<u8>,
}

impl PacketFrame {
    /// Builds a frame. Protocol constraints are checked when encoding it.
    #[must_use]
    pub const fn new(packet_id: i32, body: Vec<u8>) -> Self {
        Self {
            packet_id: MCVarInt(packet_id),
            body,
        }
    }

    /// Reads one complete uncompressed frame from a stream.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError`] when the stream ends prematurely, carries a
    /// malformed length prefix, or holds a negative packet ID.
    pub fn read(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut packet_length = 0_u32;
        let mut byte = [0_u8];
        for index in 0..3 {
            source.read_exact(&mut byte)?;
            packet_length |= u32::from(byte[0] & 0x7f) << (7 * index);
            if byte[0] & 0x80 == 0 {
                break;
            }
            if index == 2 {
                return Err(ProtocolError::Overflow);
            }
        }
        if packet_length == 0 {
            return Err(ProtocolError::InvalidData);
        }

        let packet_length =
            usize::try_from(packet_length).map_err(|_| ProtocolError::InvalidData)?;
        let mut payload = vec![0_u8; packet_length];
        source.read_exact(&mut payload)?;

        let mut payload = Cursor::new(payload);
        let packet_id = MCVarInt::unpack(&mut payload)?;
        if packet_id.0 < 0 {
            return Err(ProtocolError::InvalidData);
        }

        let body_start =
            usize::try_from(payload.position()).map_err(|_| ProtocolError::InvalidData)?;
        let body = payload.into_inner().split_off(body_start);
        Ok(Self { packet_id, body })
    }

    /// Writes one complete uncompressed frame to a stream.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError`] when the packet ID is negative, the frame is
    /// too large, or `destination` rejects the bytes.
    pub fn write(&self, destination: &mut dyn Write) -> Result<(), ProtocolError> {
        if self.packet_id.0 < 0 {
            return Err(ProtocolError::InvalidData);
        }

        let packet_id = self.packet_id.pack()?;
        let packet_length = packet_id
            .len()
            .checked_add(self.body.len())
            .ok_or(ProtocolError::Overflow)?;
        if packet_length > MAX_PACKET_LENGTH {
            return Err(ProtocolError::Overflow);
        }

        destination.write_all(
            &MCVarInt(i32::try_from(packet_length).map_err(|_| ProtocolError::Overflow)?).pack()?,
        )?;
        destination.write_all(&packet_id)?;
        destination.write_all(&self.body)?;
        Ok(())
    }

    /// Encodes one complete uncompressed frame into a byte vector.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError`] when the packet ID is negative or the frame
    /// is too large to encode.
    pub fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut encoded = Vec::new();
        self.write(&mut encoded)?;
        Ok(encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn empty_status_request_has_length_and_id_only() {
        let frame = PacketFrame::new(0, Vec::new());

        assert_eq!(frame.pack().unwrap(), [0x01, 0x00]);
    }

    #[test]
    fn reads_and_writes_a_complete_frame() {
        let frame = PacketFrame::new(1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let encoded = frame.pack().unwrap();

        assert_eq!(encoded, [0x09, 0x01, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(PacketFrame::read(&mut Cursor::new(encoded)).unwrap(), frame);
    }

    #[test]
    fn uses_a_multibyte_length_var_int() {
        let frame = PacketFrame::new(0, vec![0; 127]);
        let encoded = frame.pack().unwrap();

        assert_eq!(&encoded[..3], &[0x80, 0x01, 0x00]);
    }

    #[test]
    fn rejects_zero_length_frames() {
        let error = PacketFrame::read(&mut Cursor::new([0x00])).unwrap_err();

        assert!(matches!(error, ProtocolError::InvalidData));
    }

    #[test]
    fn rejects_packet_length_var_ints_over_three_bytes() {
        let error = PacketFrame::read(&mut Cursor::new([0x80, 0x80, 0x80])).unwrap_err();

        assert!(matches!(error, ProtocolError::Overflow));
    }

    #[test]
    fn packet_limit_includes_the_encoded_id() {
        let frame = PacketFrame::new(128, vec![0; MAX_PACKET_LENGTH - 2]);
        let encoded = frame.pack().unwrap();

        assert_eq!(&encoded[..5], &[0xff, 0xff, 0x7f, 0x80, 0x01]);
        assert_eq!(encoded.len(), MAX_PACKET_LENGTH + 3);
        assert!(matches!(
            PacketFrame::new(128, vec![0; MAX_PACKET_LENGTH - 1]).pack(),
            Err(ProtocolError::Overflow)
        ));
    }

    #[test]
    fn rejects_invalid_frames_before_writing() {
        for frame in [
            PacketFrame::new(-1, Vec::new()),
            PacketFrame::new(0, vec![0; MAX_PACKET_LENGTH]),
        ] {
            let mut destination = vec![0xaa];
            assert!(frame.write(&mut destination).is_err());
            assert_eq!(destination, [0xaa]);
            assert!(frame.pack().is_err());
        }
    }
}
