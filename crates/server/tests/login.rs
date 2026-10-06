use std::time::Duration;

use mclib::packets::configuration::{KnownPacks, RegistryData, UpdateTags};
use mclib::packets::handshaking::serverbound::{Handshake, intent};
use mclib::packets::login::clientbound::{Disconnect, LoginSuccess};
use mclib::packets::login::serverbound::{LoginAcknowledged, LoginStart};
use mclib::packets::play::clientbound::{
    ChunkDataAndUpdateLight, EmptyChunkSection, GameEvent, Login, SetDefaultSpawnPosition,
    SynchronizePlayerPosition,
};
use mclib::packets::play::serverbound::ConfirmTeleportation;
use mclib::{MCType, PacketFrame};
use server::config::StatusConfig;
use server::connection::Connection;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

async fn connect(version: i32) -> TcpStream {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let _ = Connection::new(
            stream,
            StatusConfig {
                motd: String::new(),
                max_players: 0,
            },
        )
        .run()
        .await;
    });
    let mut stream = TcpStream::connect(address).await.unwrap();
    let body = Handshake {
        protocol_version: version.into(),
        server_address: "localhost".into(),
        server_port: 25565,
        intent: intent::LOGIN.into(),
    }
    .pack()
    .unwrap();
    PacketFrame::new(0, body).write(&mut stream).await.unwrap();
    stream
}

async fn start(stream: &mut TcpStream, name: &str) {
    PacketFrame::new(
        0,
        LoginStart {
            name: name.into(),
            player_uuid: uuid::Uuid::nil().into(),
        }
        .pack()
        .unwrap(),
    )
    .write(stream)
    .await
    .unwrap();
}

async fn reply(stream: &mut TcpStream) -> PacketFrame {
    timeout(Duration::from_secs(2), PacketFrame::read(stream))
        .await
        .unwrap()
        .unwrap()
}

async fn closes(stream: &mut TcpStream) {
    assert_eq!(
        timeout(Duration::from_secs(2), stream.read(&mut [0]))
            .await
            .unwrap()
            .unwrap(),
        0
    );
}

#[tokio::test]
async fn completes_login_and_vanilla_configuration() {
    let mut stream = connect(777).await;
    start(&mut stream, "Notch").await;
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 2);
    let success = LoginSuccess::unpack(&mut frame.body.as_slice()).unwrap();
    assert_eq!(success.profile.username.as_ref(), "Notch");
    assert_eq!(
        success.profile.uuid.0.to_string(),
        "b50ad385-829d-3141-a216-7e7d7539ba7f"
    );
    assert!(success.profile.properties.0.is_empty());
    assert_eq!(success.session_id.0.get_version_num(), 4);
    PacketFrame::new(3, LoginAcknowledged.pack().unwrap())
        .write(&mut stream)
        .await
        .unwrap();
    let known = reply(&mut stream).await;
    assert_eq!(known.packet_id.0, 15);
    let packs = KnownPacks::unpack(&mut known.body.as_slice()).unwrap();
    assert_eq!(packs.packs.0.len(), 1);
    assert_eq!(packs.packs.0[0].version.as_ref(), "26.3");
    // Settings and plugin messages can precede the Known Packs response.
    PacketFrame::new(
        2,
        [b"\x0fminecraft:brand".as_slice(), b"\x07vanilla"].concat(),
    )
    .write(&mut stream)
    .await
    .unwrap();
    PacketFrame::new(7, packs.pack().unwrap())
        .write(&mut stream)
        .await
        .unwrap();
    let mut registries = 0;
    let mut entries = 0;
    let mut has_overworld = false;
    let mut has_void = false;
    let mut has_tags = false;
    let mut item_registries = std::collections::BTreeMap::new();
    loop {
        let frame = reply(&mut stream).await;
        match frame.packet_id.0 {
            7 => {
                let data = RegistryData::unpack(&mut frame.body.as_slice()).unwrap();
                registries += 1;
                entries += data.entries.0.len();
                if [
                    "minecraft:trim_material",
                    "minecraft:jukebox_song",
                    "minecraft:decorated_pot_pattern",
                    "minecraft:instrument",
                ]
                .contains(&data.registry_id.as_ref())
                {
                    item_registries.insert(
                        data.registry_id.0.clone(),
                        data.entries
                            .0
                            .iter()
                            .map(|entry| entry.id.0.clone())
                            .collect::<Vec<_>>(),
                    );
                }
                assert!(data.entries.0.iter().all(|entry| entry.data.is_none()));
                if data.registry_id.as_ref() == "minecraft:dimension_type" {
                    assert_eq!(data.entries.0[0].id.as_ref(), "minecraft:overworld");
                    has_overworld = true;
                }
                if data.registry_id.as_ref() == "minecraft:worldgen/biome" {
                    assert!(
                        data.entries
                            .0
                            .iter()
                            .any(|entry| entry.id.as_ref() == "minecraft:the_void")
                    );
                    has_void = true;
                }
            }
            13 => {}
            // Protocol 777: 0x0E is Update Tags; 0x0C is Transfer.
            14 => {
                let tags = UpdateTags::unpack(&mut frame.body.as_slice()).unwrap();
                assert_eq!(tags.registries.0.len(), 15);
                assert_eq!(
                    tags.registries
                        .0
                        .iter()
                        .map(|registry| registry.tags.0.len())
                        .sum::<usize>(),
                    773
                );
                let registry = |id: &str| {
                    tags.registries
                        .0
                        .iter()
                        .find(|registry| registry.registry_id.as_ref() == id)
                        .unwrap()
                };
                let block_tags = registry("minecraft:block");
                let infiniburn = block_tags
                    .tags
                    .0
                    .iter()
                    .find(|tag| tag.name.as_ref() == "minecraft:infiniburn_overworld")
                    .unwrap();
                // Official 26.3 Registry Dump: netherrack's block ID is 334.
                assert_eq!(infiniburn.entries.0, vec![334.into(), 729.into()]);
                let damage_tags = registry("minecraft:damage_type");
                assert!(
                    damage_tags
                        .tags
                        .0
                        .iter()
                        .find(|tag| tag.name.as_ref() == "minecraft:is_fire")
                        .unwrap()
                        .entries
                        .0
                        .len()
                        > 1
                );
                assert_eq!(registry("minecraft:banner_pattern").tags.0.len(), 11);
                has_tags = true;
            }
            3 => {
                assert!(frame.body.is_empty());
                break;
            }
            other => panic!("unexpected configuration packet {other}"),
        }
    }
    assert_eq!(registries, 32);
    assert_eq!(entries, 432);
    assert_eq!(item_registries["minecraft:trim_material"].len(), 11);
    assert!(item_registries["minecraft:trim_material"].contains(&"minecraft:redstone".to_owned()));
    assert_eq!(item_registries["minecraft:jukebox_song"].len(), 22);
    assert_eq!(item_registries["minecraft:decorated_pot_pattern"].len(), 23);
    assert_eq!(item_registries["minecraft:instrument"].len(), 8);
    assert!(
        item_registries["minecraft:instrument"].contains(&"minecraft:ponder_goat_horn".to_owned())
    );
    assert!(has_overworld && has_void);
    assert!(has_tags);
    PacketFrame::new(3, Vec::new())
        .write(&mut stream)
        .await
        .unwrap();
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0x32);
    let login = Login::unpack(&mut frame.body.as_slice()).unwrap();
    assert_eq!(login.dimension_type.0, 0);
    assert_eq!(login.dimension_name.as_ref(), "minecraft:overworld");
    assert_eq!(login.game_mode.0, 3);
    assert!(!login.online_mode.0);
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0x49);
    let position = SynchronizePlayerPosition::unpack(&mut frame.body.as_slice()).unwrap();
    assert_eq!(position.flags.0, 0);
    assert_eq!(position.y.0, 100.0);
    // Routine setup messages may precede the teleport acknowledgement.
    PacketFrame::new(0x0D, Vec::new())
        .write(&mut stream)
        .await
        .unwrap();
    PacketFrame::new(
        0,
        ConfirmTeleportation {
            teleport_id: position.teleport_id,
            x: position.x,
            y: position.y,
            z: position.z,
            yaw: position.yaw,
            pitch: position.pitch,
        }
        .pack()
        .unwrap(),
    )
    .write(&mut stream)
    .await
    .unwrap();
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0x63);
    let spawn = SetDefaultSpawnPosition::unpack(&mut frame.body.as_slice()).unwrap();
    assert_eq!(spawn.location.y, 100);
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0x27);
    assert_eq!(
        GameEvent::unpack(&mut frame.body.as_slice())
            .unwrap()
            .event
            .0,
        13
    );
    assert_eq!(reply(&mut stream).await.packet_id.0, 0x60);
    assert_eq!(reply(&mut stream).await.packet_id.0, 0x0C);
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0x2E);
    let mut body = frame.body.as_slice();
    let chunk = ChunkDataAndUpdateLight::unpack(&mut body).unwrap();
    assert!(body.is_empty());
    assert_eq!((chunk.x.0, chunk.z.0), (0, 0));
    assert_eq!(chunk.heightmaps.0.len(), 3);
    assert!(
        chunk
            .heightmaps
            .0
            .iter()
            .all(|map| map.data.0.len() == 37 && map.data.0.iter().all(|height| height.0 == 0))
    );
    let mut sections = chunk.data.0.as_slice();
    for _ in 0..24 {
        assert_eq!(
            EmptyChunkSection::unpack(&mut sections).unwrap().biome.0,
            59
        );
    }
    assert!(sections.is_empty());
    assert_eq!(chunk.sky_light.0.len(), 26);
    assert!(
        chunk
            .sky_light
            .0
            .iter()
            .all(|layer| layer.0 == vec![255; 2048])
    );
    assert!((0..26).all(|layer| chunk.sky_light_mask.get(layer) && chunk.empty_block_light_mask.get(layer)));
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0x0B);
    assert_eq!(frame.body, [1]);
    closes(&mut stream).await;
}

#[tokio::test]
async fn rejects_a_client_without_the_required_core_pack() {
    let mut stream = connect(777).await;
    start(&mut stream, "Notch").await;
    assert_eq!(reply(&mut stream).await.packet_id.0, 2);
    PacketFrame::new(3, Vec::new())
        .write(&mut stream)
        .await
        .unwrap();
    assert_eq!(reply(&mut stream).await.packet_id.0, 15);
    PacketFrame::new(7, vec![0])
        .write(&mut stream)
        .await
        .unwrap();
    let rejection = reply(&mut stream).await;
    assert_eq!(rejection.packet_id.0, 2);
    let reason = mclib::nbt::Nbt::decode_network(&mut rejection.body.as_slice()).unwrap();
    assert!(matches!(reason, mclib::nbt::Nbt::Compound(_)));
    closes(&mut stream).await;
}

#[tokio::test]
async fn rejects_incompatible_login_version_with_a_reason() {
    let mut stream = connect(764).await;
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0);
    let disconnect = Disconnect::unpack(&mut frame.body.as_slice()).unwrap();
    assert!(disconnect.reason.as_ref().contains("777"));
    closes(&mut stream).await;
}

#[tokio::test]
async fn rejects_an_invalid_username() {
    let mut stream = connect(777).await;
    start(&mut stream, "bad name").await;
    assert_eq!(reply(&mut stream).await.packet_id.0, 0);
    closes(&mut stream).await;
}

#[tokio::test]
async fn rejects_nonempty_login_acknowledgement() {
    let mut stream = connect(777).await;
    start(&mut stream, "Notch").await;
    assert_eq!(reply(&mut stream).await.packet_id.0, 2);
    PacketFrame::new(3, vec![0])
        .write(&mut stream)
        .await
        .unwrap();
    closes(&mut stream).await;
}
