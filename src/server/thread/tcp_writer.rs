use crate::server::communicator::ReadCommunicator;
use crate::server::net::tcp::{NativeWrite, TCPWrite};
use std::collections::HashMap;

#[derive(Clone)]
pub enum TCPWriterAPI {
    NewConnection {
        uid: u128,
        tcp_write: TCPWrite<NativeWrite>,
    },
}

pub struct TCPWriterThread {
    sockets: HashMap<u128, TCPWrite<NativeWrite>>,
    tcp_writer_api: ReadCommunicator<TCPWriterAPI>,
}

impl TCPWriterThread {
    pub fn new(tcp_writer_api: ReadCommunicator<TCPWriterAPI>) -> Self {
        Self {
            sockets: HashMap::new(),
            tcp_writer_api,
        }
    }

    pub fn execute(&mut self) {
        loop {
            match self.tcp_writer_api.recv() {
                TCPWriterAPI::NewConnection { uid, tcp_write } => {
                    self.sockets.insert(uid, tcp_write);
                }
            }
        }
    }
}
