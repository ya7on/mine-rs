use std::io;
use std::io::Error;

pub type MResult<R, E = MError> = Result<R, E>;

#[derive(Debug)]
pub enum MError {
    SocketError(String),
    MPSCError(String),
    TypeValidationError(String),
}

impl From<io::Error> for MError {
    fn from(value: Error) -> Self {
        // TODO
        // match value.kind() {
        //     ErrorKind::NotFound => {}
        //     ErrorKind::PermissionDenied => {}
        //     ErrorKind::ConnectionRefused => {}
        //     ErrorKind::ConnectionReset => {}
        //     ErrorKind::HostUnreachable => {}
        //     ErrorKind::NetworkUnreachable => {}
        //     ErrorKind::ConnectionAborted => {}
        //     ErrorKind::NotConnected => {}
        //     ErrorKind::AddrInUse => {}
        //     ErrorKind::AddrNotAvailable => {}
        //     ErrorKind::NetworkDown => {}
        //     ErrorKind::BrokenPipe => {}
        //     ErrorKind::AlreadyExists => {}
        //     ErrorKind::WouldBlock => {}
        //     ErrorKind::NotADirectory => {}
        //     ErrorKind::IsADirectory => {}
        //     ErrorKind::DirectoryNotEmpty => {}
        //     ErrorKind::ReadOnlyFilesystem => {}
        //     ErrorKind::FilesystemLoop => {}
        //     ErrorKind::StaleNetworkFileHandle => {}
        //     ErrorKind::InvalidInput => {}
        //     ErrorKind::InvalidData => {}
        //     ErrorKind::TimedOut => {}
        //     ErrorKind::WriteZero => {}
        //     ErrorKind::StorageFull => {}
        //     ErrorKind::NotSeekable => {}
        //     ErrorKind::FilesystemQuotaExceeded => {}
        //     ErrorKind::FileTooLarge => {}
        //     ErrorKind::ResourceBusy => {}
        //     ErrorKind::ExecutableFileBusy => {}
        //     ErrorKind::Deadlock => {}
        //     ErrorKind::CrossesDevices => {}
        //     ErrorKind::TooManyLinks => {}
        //     ErrorKind::InvalidFilename => {}
        //     ErrorKind::ArgumentListTooLong => {}
        //     ErrorKind::Interrupted => {}
        //     ErrorKind::Unsupported => {}
        //     ErrorKind::UnexpectedEof => {}
        //     ErrorKind::OutOfMemory => {}
        //     ErrorKind::Other => {}
        //     ErrorKind::Uncategorized => {}
        // };
        MError::SocketError(value.to_string())
    }
}
