use crate::io::tcp::TcpOutput;
use crate::io::tcp_facade::TcpFacadeInput;
use common::error::{MError, MResult};
use common::tracing::{error, info};
use packets::from_client::login::login_start::LoginStartPacket;
use packets::from_client::status::ping::PingRequest;
use packets::from_server::login::login_success::LoginSuccessPacket;
use packets::from_server::status::status_response::{
    Description, Players, StatusResponsePacket, Version,
};
use packets::handshake::HandshakePacket;
use packets::MinecraftPacket;
use std::net::SocketAddr;
use types::{MinecraftUUID, MinecraftVarInt};
use uuid::Uuid;

enum HandshakeNextStep {
    Status,
    Login,
}

pub struct Session {
    pub session_id: Uuid,
    io: TcpOutput,
    addr: SocketAddr,
}

impl Session {
    pub fn new(io: TcpOutput, addr: SocketAddr) -> Self {
        let session_id = Uuid::new_v4();
        Self {
            session_id,
            io,
            addr,
        }
    }

    async fn handle_handshake(&mut self) -> MResult<HandshakeNextStep> {
        let (_len, _packet_id, mut packet) = self.io.next_packet().await?;
        let handshake = HandshakePacket::parse_from(&mut packet).await?;
        match handshake.next_state.0 {
            1 => Ok(HandshakeNextStep::Status),
            2 => Ok(HandshakeNextStep::Login),
            _ => {
                return Err(MError::TypeValidationError(format!(
                    "Invalid next_step value"
                )))
            }
        }
    }

    async fn handle_status(&mut self, tcp_facade_tx: &TcpFacadeInput) -> MResult<()> {
        let (_len, _packet_id, _packet) = self.io.next_packet().await?;

        let status_response = StatusResponsePacket {
            version: Version {
                name: "Mine.rs".to_string(),
                protocol: 759,
            },
            players: Players { max: 25, online: 0 },
            description: Description {
                text: "Hello World".to_string(),
            },
        };
        let packet = status_response.to_packet(MinecraftVarInt(0x00)).await?;

        tcp_facade_tx.send_to(self.session_id, packet).await?;

        let (_len, _packet_id, mut packet) = self.io.next_packet().await?;
        let ping = PingRequest::parse_from(&mut packet).await?;
        let packet = ping.to_packet(MinecraftVarInt(0x01)).await?;

        tcp_facade_tx.send_to(self.session_id, packet).await?;

        Ok(())
    }

    async fn handle_login(&mut self, tcp_facade_tx: &TcpFacadeInput) -> MResult<()> {
        let (_len, _packet_id, mut packet) = self.io.next_packet().await?;
        let login_start = LoginStartPacket::parse_from(&mut packet).await?;

        let login_success = LoginSuccessPacket {
            uuid: MinecraftUUID(self.session_id),
            username: login_start.name,
            number_of_properties: MinecraftVarInt(0),
            properties: vec![],
        };
        let packet = login_success.to_packet(MinecraftVarInt(0x02)).await?;

        tcp_facade_tx.send_to(self.session_id, packet).await?;

        Ok(())
    }

    async fn handle_play(&mut self, tcp_facade_tx: &TcpFacadeInput) -> MResult<()> {
        Ok(())
    }

    async fn try_run(&mut self, tcp_facade_tx: &TcpFacadeInput) -> MResult<()> {
        match self.handle_handshake().await? {
            HandshakeNextStep::Status => self.handle_status(tcp_facade_tx).await?,
            HandshakeNextStep::Login => {
                self.handle_login(tcp_facade_tx).await?;
                self.handle_play(tcp_facade_tx).await?;
            }
        };
        Ok(())
    }

    pub async fn run(&mut self, tcp_facade_tx: TcpFacadeInput) -> MResult<()> {
        match self.try_run(&tcp_facade_tx).await {
            Ok(_) => {
                info!("Socket successfully closed");
            }
            Err(err) => {
                error!("Socket closed with error: {:?}", err)
            }
        };
        tcp_facade_tx.remove_session(self.session_id).await?;
        Ok(())
    }
}
