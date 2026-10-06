//! Uncompressed Minecraft packet framing.

#[cfg(feature = "tokio-io")]
use std::io::Cursor;

#[cfg(feature = "tokio-io")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

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
    /// Cancelling a partial read requires closing the stream; restarting loses framing.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError`] when the stream ends prematurely, carries a
    /// malformed length prefix, or holds a negative packet ID.
    #[cfg(feature = "tokio-io")]
    pub async fn read<R: AsyncRead + Unpin + ?Sized>(
        source: &mut R,
    ) -> Result<Self, ProtocolError> {
        let mut packet_length = 0_u32;
        let mut byte = [0_u8];
        for index in 0..3 {
            source.read_exact(&mut byte).await?;
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
        source.read_exact(&mut payload).await?;

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
    #[cfg(feature = "tokio-io")]
    pub async fn write<W: AsyncWrite + Unpin + ?Sized>(
        &self,
        destination: &mut W,
    ) -> Result<(), ProtocolError> {
        destination.write_all(&self.pack()?).await?;
        Ok(())
    }

    /// Encodes one complete uncompressed frame into a byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error when the packet ID is negative or the frame is too large.
    pub fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
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

        let mut encoded =
            MCVarInt(i32::try_from(packet_length).map_err(|_| ProtocolError::Overflow)?).pack()?;
        encoded.extend_from_slice(&packet_id);
        encoded.extend_from_slice(&self.body);
        Ok(encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "tokio-io")]
    use tokio::io::duplex;

    #[test]
    fn empty_status_request_has_length_and_id_only() {
        let frame = PacketFrame::new(0, Vec::new());

        assert_eq!(frame.pack().unwrap(), [0x01, 0x00]);
    }

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn reads_and_writes_a_complete_frame() {
        let frame = PacketFrame::new(1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let encoded = frame.pack().unwrap();

        assert_eq!(encoded, [0x09, 0x01, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            PacketFrame::read(&mut encoded.as_slice()).await.unwrap(),
            frame
        );
    }

    #[test]
    fn uses_a_multibyte_length_var_int() {
        let frame = PacketFrame::new(0, vec![0; 127]);
        let encoded = frame.pack().unwrap();

        assert_eq!(&encoded[..3], &[0x80, 0x01, 0x00]);
    }

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn rejects_zero_length_frames() {
        let error = PacketFrame::read(&mut [0x00].as_slice()).await.unwrap_err();

        assert!(matches!(error, ProtocolError::InvalidData));
    }

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn rejects_packet_length_var_ints_over_three_bytes() {
        let error = PacketFrame::read(&mut [0x80, 0x80, 0x80].as_slice())
            .await
            .unwrap_err();

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

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn rejects_invalid_frames_before_writing() {
        for frame in [
            PacketFrame::new(-1, Vec::new()),
            PacketFrame::new(0, vec![0; MAX_PACKET_LENGTH]),
        ] {
            let mut destination = vec![0xaa];
            assert!(frame.write(&mut destination).await.is_err());
            assert_eq!(destination, [0xaa]);
            assert!(frame.pack().is_err());
        }
    }
    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn reads_fragmented_length_and_payload() {
        let expected = PacketFrame::new(1, vec![42; 128]);
        let bytes = expected.pack().unwrap();
        let (mut writer, mut reader) = duplex(1);
        let sending = tokio::spawn(async move {
            for byte in bytes {
                writer.write_all(&[byte]).await.unwrap();
                tokio::task::yield_now().await;
            }
        });
        assert_eq!(PacketFrame::read(&mut reader).await.unwrap(), expected);
        sending.await.unwrap();
    }

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn leaves_the_next_frame_for_the_next_read() {
        let (mut writer, mut reader) = duplex(32);
        writer.write_all(&[1, 0, 2, 1, 42]).await.unwrap();
        assert_eq!(
            PacketFrame::read(&mut reader).await.unwrap(),
            PacketFrame::new(0, vec![])
        );
        assert_eq!(
            PacketFrame::read(&mut reader).await.unwrap(),
            PacketFrame::new(1, vec![42])
        );
    }

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn rejects_invalid_lengths_without_waiting_for_payload() {
        for (bytes, overflow) in [(vec![0], false), (vec![0x80; 3], true)] {
            let (mut writer, mut reader) = duplex(8);
            writer.write_all(&bytes).await.unwrap();
            let error = PacketFrame::read(&mut reader).await.unwrap_err();
            if overflow {
                assert!(matches!(error, ProtocolError::Overflow));
            } else {
                assert!(matches!(error, ProtocolError::InvalidData));
            }
        }
    }

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn reports_eof_inside_payload() {
        let (mut writer, mut reader) = duplex(8);
        writer.write_all(&[3, 0, 42]).await.unwrap();
        drop(writer);
        assert!(matches!(PacketFrame::read(&mut reader).await.unwrap_err(),
            ProtocolError::Io(error) if error.kind() == std::io::ErrorKind::UnexpectedEof));
    }

    #[cfg(feature = "tokio-io")]
    #[tokio::test]
    async fn rejects_invalid_packet_ids() {
        for bytes in [
            vec![5, 0xff, 0xff, 0xff, 0xff, 0x0f],
            vec![1, 0x80],
            vec![5, 0x80, 0x80, 0x80, 0x80, 0x80],
        ] {
            assert!(PacketFrame::read(&mut bytes.as_slice()).await.is_err());
        }
    }
}
