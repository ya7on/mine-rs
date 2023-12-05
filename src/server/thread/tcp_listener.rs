use crate::conf::conf;
use crate::registry;
use crate::server::communicator::WriteCommunicator;
use crate::server::net::tcp::{NativeRead, TCPRead};
use crate::server::thread::tcp_writer::TCPWriterAPI;
use mclib::chunk_format::{ChunkData, ChunkSection, DataArray, PalletedContainer};
use mclib::nbt::{IntoNBTTag, NBT};
use mclib::packets::client::{
    ChunkDataAndUpdateLight, ClientboundKeelAlivePlay, FinishConfigurationClientbound,
    LoginSuccess, Play, RegistryData, SetDefaultSpawnPosition, StatusResponse,
    SynchronizePlayerPosition,
};
use mclib::packets::server::{
    FinishConfigurationServerbound, Handshake, HandshakeNextState, LoginAcknowledged, LoginStart,
    PingRequest, ServerboundKeelAlivePlay, SetPlayerPosition, StatusRequest,
};
use mclib::types::{MCByteArray, MCLong, MCPosition, MCVarInt};
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
            registry_data: registry_data_nbt.into(),
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

    pub fn handle_play(&mut self) {
        let play = Play {
            entity_id: 0.into(),
            is_hardcore: false.into(),
            dimensions: vec![],
            max_players: 25.into(),
            view_distance: 32.into(),
            simulation_distance: 32.into(),
            reduced_debug_info: false.into(),
            enable_respawn_screen: true.into(),
            do_limited_crafting: true.into(),
            dimension_type: "minecraft:overworld".into(),
            dimension_name: "minecraft:overworld".into(),
            hashed_seed: 0.into(),
            game_mode: 1.into(),
            previous_game_mode: 1.into(),
            is_debug: false.into(),
            is_flat: true.into(),
            death_info: None,
            portal_cooldown: 1.into(),
        };
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: play.pack(),
        });

        let synchronize_player_position = SynchronizePlayerPosition {
            x: 0.0.into(),
            y: 100.0.into(),
            z: 0.0.into(),
            yaw: 0.0.into(),
            pitch: 0.0.into(),
            flags: 0.into(),
            teleport_id: 0.into(),
        };
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: synchronize_player_position.pack(),
        });

        let set_default_spawn_position = SetDefaultSpawnPosition {
            location: MCPosition { x: 0, y: 100, z: 0 },
            angle: 0.0.into(),
        };
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: set_default_spawn_position.pack(),
        });

        for x in -3..3 {
            for z in -3..3 {
                let heightmap = NBT(
                    None,
                    vec![
                        (
                            "MOTION_BLOCKING",
                            DataArray::from(vec![4; 256]).pack(9).to_nbt(),
                        ),
                        (
                            "WORLD_SURFACE",
                            DataArray::from(vec![4; 256]).pack(9).to_nbt(),
                        ),
                    ]
                    .to_nbt(),
                );

                let mut data = Vec::new();
                data.extend(vec![1; 256]);
                data.extend(vec![2; 512]);
                data.extend(vec![3; 256]);
                data.extend(vec![0; 3072]);
                let mut chunks = Vec::new();
                chunks.push(ChunkSection {
                    block_count: 1024.into(),
                    block_states: PalletedContainer::Indirect {
                        bits_per_entry: 4.into(),
                        pallete: vec![0.into(), 79.into(), 10.into(), 9.into()],
                        data: DataArray::from(data)
                            .pack(4)
                            .iter()
                            .map(|i| (*i).into())
                            .collect::<Vec<MCLong>>(),
                    },
                    biomes: PalletedContainer::SingleValued(39.into()),
                });
                chunks.extend(vec![
                    ChunkSection {
                        block_count: 0.into(),
                        block_states: PalletedContainer::SingleValued(0.into()),
                        biomes: PalletedContainer::SingleValued(39.into()),
                    };
                    23
                ]);
                let chunk_data_and_update_light = ChunkDataAndUpdateLight {
                    chunk_x: x.into(),
                    chunk_z: z.into(),
                    heightmaps: heightmap.into(),
                    data: MCByteArray::new(ChunkData(chunks)),
                    block_entities: vec![],
                    sky_light_mask: Default::default(),
                    block_light_mask: Default::default(),
                    empty_sky_light_mask: Default::default(),
                    empty_block_light_mask: Default::default(),
                    sky_lights: vec![],
                    block_lights: vec![],
                };
                self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
                    uid: self.uid,
                    body: chunk_data_and_update_light.pack(),
                });
            }
        }

        let keepalive = ClientboundKeelAlivePlay {
            keepalive_id: 14.into(),
        };
        self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
            uid: self.uid,
            body: keepalive.pack(),
        });

        loop {
            let mut packet = self.tcp_read.read_packet();

            match packet.id {
                0x16 => {
                    println!("{:?}", packet.parse_packet::<SetPlayerPosition>());

                    let keepalive = ClientboundKeelAlivePlay {
                        keepalive_id: 14.into(),
                    };
                    self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
                        uid: self.uid,
                        body: keepalive.pack(),
                    });
                }
                0x14 => {
                    let keepalive_id: i64 = packet
                        .parse_packet::<ServerboundKeelAlivePlay>()
                        .keepalive_id
                        .into();
                    println!("{:?}", keepalive_id);

                    let keepalive = ClientboundKeelAlivePlay {
                        keepalive_id: (keepalive_id + 1).into(),
                    };
                    self.tcp_writer_api.send(TCPWriterAPI::SendMessageRaw {
                        uid: self.uid,
                        body: keepalive.pack(),
                    });
                }
                _ => {}
            }
        }
    }

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
