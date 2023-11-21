use crate::conf::conf;
use crate::server::communicator::WriteCommunicator;
use crate::server::net::tcp::{NativeRead, TCPRead};
use crate::server::thread::tcp_writer::TCPWriterAPI;
use mclib::packets::client::StatusResponse;
use mclib::packets::server::{Handshake, HandshakeNextState, PingRequest, StatusRequest};
use mclib::types::MCVarInt;
use mclib::MCPacket;

pub struct TCPListenerThread {
    uid: u128,
    tcp_read: TCPRead<NativeRead>,
    tcp_writer_api: WriteCommunicator<TCPWriterAPI>,
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
        }
    }

    pub fn handle_handshake(&mut self) -> HandshakeNextState {
        let (_, _, handshake) = self.tcp_read.read_specific_packet::<Handshake>();
        let next_state = handshake.next_state;
        HandshakeNextState::from(<MCVarInt as Into<i32>>::into(next_state))
    }

    pub fn handle_status(&mut self) {
        let (_, _, _) = self.tcp_read.read_specific_packet::<StatusRequest>();
        let c = conf();
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: StatusResponse {
                json_response: format!(r#"{{"version":{{"name":"{version}","protocol":764}},"players":{{"max":{max_players},"online":0}},"description":{{"text":"{motd}"}}}}"#, version=c.app_name, max_players=c.max_players, motd=c.motd).into(),
            }
            .pack(),
        });

        let (_, _, ping) = self.tcp_read.read_specific_packet::<PingRequest>();
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: ping.pack(),
        });
    }

    pub fn execute(&mut self) {
        match self.handle_handshake() {
            HandshakeNextState::Status => {
                self.handle_status();
            }
            HandshakeNextState::Login => {
                unimplemented!()
            }
            HandshakeNextState::Unknown => {
                error!("Unknown next state for handshake");
                todo!() // Add shutdown
            }
        }
        debug!("End connection");
    }
}
