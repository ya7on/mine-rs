use crate::io::reader::IOReader;
use crate::io::writer::IOWriter;
use crate::session::Session;
use common::error::{MError, MResult};
use common::tracing::{debug, error};
use std::fmt::Debug;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, ToSocketAddrs};

pub struct Listener {
    io: TcpListener,
}

impl Listener {
    pub async fn new<A: ToSocketAddrs + Debug>(addr: A) -> MResult<Self> {
        debug!("Creating TCP listener for {:?}", addr);
        let io = TcpListener::bind(addr).await.map_err(|err| {
            error!("TCP listener bind failed: {}", err);
            MError::from(err)
        })?;

        Ok(Self { io })
    }

    pub async fn accept(&self) -> MResult<(Session, IOWriter<OwnedWriteHalf>)> {
        let (io, addr) = self.io.accept().await.map_err(|err| {
            error!("TCP connection accept failed: {}", err);
            MError::from(err)
        })?;

        let (owned_read_half, owned_write_half) = io.into_split();
        let tcp_reader = IOReader::from(owned_read_half);
        let tcp_writer = IOWriter::from(owned_write_half);

        Ok((Session::new(tcp_reader, addr), tcp_writer))
    }
}
