use crate::server::communicator::WriteCommunicator;
use crate::server::net::tcp::{NativeRead, TCPRead};
use crate::server::thread::tcp_writer::TCPWriterAPI;
use mclib::{Handshake, HandshakeNextState, MCVarInt};

pub struct TCPListenerThread {
    uid: u128,
    tcp_read: TCPRead<NativeRead>,
    tcp_writer_api: WriteCommunicator<TCPWriterAPI>,
    compression_treshold: Option<i32>,
}

impl TCPListenerThread {
    pub fn new(
        uid: u128,
        tcp_read: TCPRead<NativeRead>,
        tcp_writer_api: WriteCommunicator<TCPWriterAPI>,
    ) -> Self {
        Self {
            uid,
            tcp_read,
            tcp_writer_api,
            compression_treshold: None,
        }
    }

    pub fn handle_handshake(&mut self) -> HandshakeNextState {
        let next_state = self
            .tcp_read
            .read_specific_packet::<Handshake>(self.compression_treshold)
            .next_state;
        HandshakeNextState::from(<MCVarInt as Into<i32>>::into(next_state))
    }

    pub fn handle_status(&mut self) {
        todo!()
    }

    pub fn execute(&mut self) {
        loop {
            match self.handle_handshake() {
                HandshakeNextState::Status => {}
                HandshakeNextState::Login => {
                    unimplemented!()
                }
                HandshakeNextState::Unknown => {
                    error!("Unknown next state for handshake");
                    todo!() // Add shutdown
                }
            }
        }
    }
}
