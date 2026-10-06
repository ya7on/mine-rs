use mclib::ProtocolError;

/// Errors that end a status connection.
///
/// Every variant is fatal for the connection; the protocol has no disconnect
/// packet in the status state, so recovery means closing the socket.
#[derive(Debug)]
pub enum ConnectionError {
    Io(std::io::Error),
    Protocol(ProtocolError),
    UnexpectedPacket {
        packet_id: i32,
        expected: &'static str,
    },
    HandshakeRequired,
}

impl From<std::io::Error> for ConnectionError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ProtocolError> for ConnectionError {
    fn from(error: ProtocolError) -> Self {
        Self::Protocol(error)
    }
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Protocol(error) => write!(formatter, "protocol error: {error}"),
            Self::UnexpectedPacket {
                packet_id,
                expected,
            } => {
                write!(
                    formatter,
                    "unexpected packet {packet_id}, expected {expected}"
                )
            }
            Self::HandshakeRequired => {
                formatter.write_str("connection closed before a handshake arrived")
            }
        }
    }
}

impl std::error::Error for ConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Protocol(error) => Some(error),
            _ => None,
        }
    }
}
