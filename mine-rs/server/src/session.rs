use crate::io::tcp::TcpOutput;
use crate::io::tcp_facade::TcpFacadeInput;
use common::error::{MError, MResult};
use common::tracing::{error, info};
use nbt::NBT;
use packets::from_client::login::login_start::LoginStartPacket;
use packets::from_client::status::ping::PingRequest;
use packets::from_server::login::login_success::LoginSuccessPacket;
use packets::from_server::play::login::Login;
use packets::from_server::status::status_response::{
    Description, Players, StatusResponsePacket, Version,
};
use packets::handshake::HandshakePacket;
use packets::MinecraftPacket;
use std::net::SocketAddr;
use types::{
    MinecraftBoolean, MinecraftByte, MinecraftInt, MinecraftLong, MinecraftString, MinecraftUUID,
    MinecraftUnsignedByte, MinecraftVarInt,
};
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
        let nbt = NBT::Root((
            "registry codec".to_string(),
            vec![
                (
                    "minecraft:dimension_type".to_string(),
                    NBT::Compound(vec![
                        (
                            "type".to_string(),
                            NBT::String("minecraft:dimension_type".to_string()),
                        ),
                        (
                            "value".to_string(),
                            NBT::List(vec![NBT::Compound(vec![
                                (
                                    "name".to_string(),
                                    NBT::String("minecraft:overworld".to_string()),
                                ),
                                ("id".to_string(), NBT::Int(0)),
                                (
                                    "element".to_string(),
                                    NBT::Compound(vec![
                                        ("piglin_safe".to_string(), NBT::Byte(0)),
                                        ("has_raids".to_string(), NBT::Byte(0)),
                                        ("monster_spawn_light_level".to_string(), NBT::Int(11)),
                                        (
                                            "monster_spawn_block_light_limit".to_string(),
                                            NBT::Int(0),
                                        ),
                                        ("natural".to_string(), NBT::Byte(1)),
                                        ("ambient_light".to_string(), NBT::Float(0.1)),
                                        ("fixed_time".to_string(), NBT::Long(0)),
                                        (
                                            "infiniburn".to_string(),
                                            NBT::String(
                                                "#minecraft:infiniburn_overworld".to_string(),
                                            ),
                                        ),
                                        ("respawn_anchor_works".to_string(), NBT::Byte(0)),
                                        ("has_skylight".to_string(), NBT::Byte(1)),
                                        ("bed_works".to_string(), NBT::Byte(1)),
                                        (
                                            "effects".to_string(),
                                            NBT::String("minecraft:overworld".to_string()),
                                        ),
                                        ("min_y".to_string(), NBT::Int(-64)),
                                        ("height".to_string(), NBT::Int(256)),
                                        ("logical_height".to_string(), NBT::Int(128)),
                                        ("coordinate_scale".to_string(), NBT::Double(1.0)),
                                        ("ultrawarm".to_string(), NBT::Byte(0)),
                                        ("has_ceiling".to_string(), NBT::Byte(0)),
                                    ]),
                                ),
                            ])]),
                        ),
                    ]),
                ),
                (
                    "minecraft:worldgen/biome".to_string(),
                    NBT::Compound(vec![
                        (
                            "type".to_string(),
                            NBT::String("minecraft:worldgen/biome".to_string()),
                        ),
                        (
                            "value".to_string(),
                            NBT::List(vec![NBT::Compound(vec![
                                (
                                    "name".to_string(),
                                    NBT::String("minecraft:the_void".to_string()),
                                ),
                                ("id".to_string(), NBT::Int(0)),
                                (
                                    "element".to_string(),
                                    NBT::Compound(vec![
                                        (
                                            "precipitation".to_string(),
                                            NBT::String("none".to_string()),
                                        ),
                                        ("temperature".to_string(), NBT::Float(0.5)),
                                        ("downfall".to_string(), NBT::Float(0.5)),
                                        (
                                            "effects".to_string(),
                                            NBT::Compound(vec![
                                                ("sky_color".to_string(), NBT::Int(8103167)),
                                                ("water_fog_color".to_string(), NBT::Int(329011)),
                                                ("fog_color".to_string(), NBT::Int(12638463)),
                                                ("water_color".to_string(), NBT::Int(4159204)),
                                            ]),
                                        ),
                                    ]),
                                ),
                            ])]),
                        ),
                    ]),
                ),
                // (
                //     "minecraft:chat_type".to_string(),
                //     NBT::Compound(vec![
                //         (
                //             "type".to_string(),
                //             NBT::String("minecraft:chat_type".to_string()),
                //         ),
                //         (
                //             "value".to_string(),
                //             NBT::List(vec![NBT::Compound(vec![
                //                 (
                //                     "name".to_string(),
                //                     NBT::String("minecraft:chat".to_string()),
                //                 ),
                //                 ("id".to_string(), NBT::Int(0)),
                //                 (
                //                     "elements".to_string(),
                //                     NBT::Compound(vec![
                //                         (
                //                             "chat".to_string(),
                //                             NBT::Compound(vec![(
                //                                 "decoration".to_string(),
                //                                 NBT::Compound(vec![
                //                                     (
                //                                         "translation_key".to_string(),
                //                                         NBT::String("chat.type.text".to_string()),
                //                                     ),
                //                                     ("style".to_string(), NBT::Compound(vec![])),
                //                                     (
                //                                         "parameters".to_string(),
                //                                         NBT::List(vec![NBT::String(
                //                                             "sender".to_string(),
                //                                         )]),
                //                                     ),
                //                                 ]),
                //                             )]),
                //                         ),
                //                         (
                //                             "narration".to_string(),
                //                             NBT::Compound(vec![
                //                                 (
                //                                     "decoration".to_string(),
                //                                     NBT::Compound(vec![
                //                                         (
                //                                             "translation_key".to_string(),
                //                                             NBT::String(
                //                                                 "chat.type.text".to_string(),
                //                                             ),
                //                                         ),
                //                                         (
                //                                             "style".to_string(),
                //                                             NBT::Compound(vec![]),
                //                                         ),
                //                                         (
                //                                             "parameters".to_string(),
                //                                             NBT::List(vec![NBT::String(
                //                                                 "sender".to_string(),
                //                                             )]),
                //                                         ),
                //                                     ]),
                //                                 ),
                //                                 (
                //                                     "priority".to_string(),
                //                                     NBT::String("chat".to_string()),
                //                                 ),
                //                             ]),
                //                         ),
                //                     ]),
                //                 ),
                //             ])]),
                //         ),
                //     ]),
                // ),
            ],
        ));

        let login_play = Login {
            entity_id: MinecraftInt(1),
            is_hardcore: MinecraftBoolean(false),
            gamemode: MinecraftUnsignedByte(0),
            previous_gamemode: MinecraftByte(0),
            dimension_count: MinecraftVarInt(1),
            dimension_names: vec![MinecraftString("minecraft:world".to_string())],
            registry_codec: nbt.to_hex(),
            dimension_type: MinecraftString("minecraft:overworld".to_string()),
            dimension_name: MinecraftString("minecraft:overworld".to_string()),
            hashed_seed: MinecraftLong(0),
            max_players: MinecraftVarInt(25),
            view_distance: MinecraftVarInt(10),
            simulation_distance: MinecraftVarInt(10),
            reduced_debug_info: MinecraftBoolean(false),
            enable_respawn_screen: MinecraftBoolean(false),
            is_debug: MinecraftBoolean(false),
            is_flat: MinecraftBoolean(false),
            has_death_location: MinecraftBoolean(false),
        };

        let packet = login_play.to_packet(MinecraftVarInt(0x23)).await?;

        tcp_facade_tx.send_to(self.session_id, packet).await?;

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
