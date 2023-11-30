use crate::conf::conf;
use crate::registry;
use crate::server::communicator::WriteCommunicator;
use crate::server::net::tcp::{NativeRead, TCPRead};
use crate::server::thread::tcp_writer::TCPWriterAPI;
use mclib::nbt::NBT;
use mclib::packets::client::{
    FinishConfigurationClientbound, LoginSuccess, RegistryData, StatusResponse,
};
use mclib::packets::server::{
    FinishConfigurationServerbound, Handshake, HandshakeNextState, LoginAcknowledged, LoginStart,
    PingRequest, StatusRequest,
};
use mclib::types::{MCVarInt, MCNBT};
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
        let handshake = self.tcp_read.read_packet().parse_packet::<Handshake>();
        let next_state = handshake.next_state;
        HandshakeNextState::from(<MCVarInt as Into<i32>>::into(next_state))
    }

    pub fn handle_status(&mut self) {
        let _ = self.tcp_read.read_packet().parse_packet::<StatusRequest>();
        let c = conf();
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: StatusResponse {
                json_response: format!(r#"{{"version":{{"name":"{version}","protocol":764}},"players":{{"max":{max_players},"online":0}},"description":{{"text":"{motd}"}}}}"#, version=c.app_name, max_players=c.max_players, motd=c.motd).into(),
            }
            .pack(),
        });

        let ping = self.tcp_read.read_packet().parse_packet::<PingRequest>();
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: ping.pack(),
        });
    }

    pub fn handle_login(&mut self) {
        loop {
            let mut packet = self.tcp_read.read_packet();

            match packet.id {
                0x00 => {
                    let login_start = packet.parse_packet::<LoginStart>();
                    let login_success = LoginSuccess {
                        uuid: login_start.player_uuid,
                        username: login_start.name,
                        properties: vec![],
                    };

                    self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
                        uid: self.uid,
                        body: login_success.pack(),
                    })
                }
                0x01 => {
                    let _login_acknowledged = packet.parse_packet::<LoginAcknowledged>();
                    return;
                }
                _ => break,
            }
        }
    }

    pub fn handle_configuration(&mut self) {
        let registry_data = serde_json::from_str::<registry::RegistryData>(include_str!(
            "../../../assets/registry_data_1.20.2.json"
        ))
        .unwrap();
        let registry_data_nbt = NBT::from(registry_data);
        let registry_data_packet = RegistryData {
            registry_data: MCNBT::from(registry_data_nbt),
        };
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: registry_data_packet.pack(),
        });

        let finish_configuration = FinishConfigurationClientbound {};
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: finish_configuration.pack(),
        });

        let _ = self
            .tcp_read
            .read_packet()
            .parse_packet::<FinishConfigurationServerbound>();
    }

    pub fn handle_play(&mut self) {}

    pub fn execute(&mut self) {
        match self.handle_handshake() {
            HandshakeNextState::Status => {
                self.handle_status();
            }
            HandshakeNextState::Login => {
                self.handle_login();
                self.handle_configuration();
                self.handle_play();
            }
            HandshakeNextState::Unknown => {
                error!("Unknown next state for handshake");
                todo!() // Add shutdown
            }
        }
        debug!("End connection");
        self.tcp_read.close();
        self.tcp_writer_api
            .send(TCPWriterAPI::CloseConnection { uid: self.uid });
    }
}
