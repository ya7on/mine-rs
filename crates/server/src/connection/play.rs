use mclib::packets::play::clientbound::{
    ChunkBatchFinished, ChunkDataAndUpdateLight, EmptyChunkSection, GameEvent, Heightmap, Login,
    SetCenterChunk, SetDefaultSpawnPosition, SynchronizePlayerPosition,
};
use mclib::packets::play::serverbound::ConfirmTeleportation;
use mclib::{MCBitSet, MCPosition, MCType, ProtocolError};

use crate::ConnectionError;
use crate::connection::Connection;

pub async fn run(connection: &mut Connection) -> Result<(), ConnectionError> {
    connection
        .write_frame(
            0x32,
            Login {
                entity_id: 1.into(),
                is_hardcore: false.into(),
                dimensions: vec!["minecraft:overworld".into()].into(),
                max_players: 1.into(),
                view_distance: 2.into(),
                simulation_distance: 2.into(),
                reduced_debug_info: false.into(),
                enable_respawn_screen: true.into(),
                limited_crafting: false.into(),
                // The exported dimension registry begins with overworld.
                dimension_type: 0.into(),
                dimension_name: "minecraft:overworld".into(),
                hashed_seed: 0.into(),
                game_mode: 3.into(),
                previous_game_mode: 0.into(),
                is_debug: false.into(),
                is_flat: true.into(),
                death_location: None,
                portal_cooldown: 0.into(),
                sea_level: 63.into(),
                online_mode: false.into(),
                enforces_secure_chat: false.into(),
            }
            .pack()?,
        )
        .await?;
    let position = SynchronizePlayerPosition {
        teleport_id: 1.into(),
        x: 8.0.into(),
        y: 100.0.into(),
        z: 8.0.into(),
        velocity_x: 0.0.into(),
        velocity_y: 0.0.into(),
        velocity_z: 0.0.into(),
        yaw: 0.0.into(),
        pitch: 0.0.into(),
        flags: 0.into(),
    };
    connection.write_frame(0x49, position.pack()?).await?;
    send_chunk(connection).await?;
    loop {
        let frame = connection.read_frame().await?;
        if frame.packet_id.0 != 0 {
            // Movement and client setup messages do not affect this connection demo.
            continue;
        }
        let mut body = frame.body.as_slice();
        let confirmation = ConfirmTeleportation::unpack(&mut body)?;
        if !body.is_empty()
            || confirmation.teleport_id != position.teleport_id
            || confirmation.x != position.x
            || confirmation.y != position.y
            || confirmation.z != position.z
            || confirmation.yaw != position.yaw
            || confirmation.pitch != position.pitch
        {
            return Err(ProtocolError::InvalidData.into());
        }
        log::info!("Play teleport acknowledged by client");
        return Ok(());
    }
}

async fn send_chunk(connection: &mut Connection) -> Result<(), ConnectionError> {
    connection
        .write_frame(
            0x63,
            SetDefaultSpawnPosition {
                dimension: "minecraft:overworld".into(),
                location: MCPosition { x: 8, y: 100, z: 8 },
                yaw: 0.0.into(),
                pitch: 0.0.into(),
            }
            .pack()?,
        )
        .await?;
    connection
        .write_frame(
            0x27,
            GameEvent {
                event: 13.into(),
                value: 0.0.into(),
            }
            .pack()?,
        )
        .await?;
    connection
        .write_frame(
            0x60,
            SetCenterChunk {
                x: 0.into(),
                z: 0.into(),
            }
            .pack()?,
        )
        .await?;
    // Overworld covers -64..320: 24 sections and 26 light layers.
    // Air is static block-state ID 0; the_void is exported biome ID 59.
    let section = EmptyChunkSection { biome: 59.into() }.pack()?;
    let mut mask = MCBitSet::default();
    for layer in 0..26 {
        mask.set(layer);
    }
    let chunk = ChunkDataAndUpdateLight {
        x: 0.into(),
        z: 0.into(),
        heightmaps: [1, 4, 5]
            .into_iter()
            .map(|kind| Heightmap {
                kind: kind.into(),
                // 256 zero heights packed at nine bits, seven per long.
                data: vec![0.into(); 37].into(),
            })
            .collect::<Vec<_>>()
            .into(),
        data: section.repeat(24).into(),
        block_entities: Vec::new().into(),
        sky_light_mask: mask.clone(),
        block_light_mask: MCBitSet::default(),
        empty_sky_light_mask: MCBitSet::default(),
        empty_block_light_mask: mask,
        sky_light: vec![vec![255; 2048].into(); 26].into(),
        block_light: Vec::new().into(),
    };
    connection.write_frame(0x0C, Vec::new()).await?;
    connection.write_frame(0x2E, chunk.pack()?).await?;
    connection
        .write_frame(0x0B, ChunkBatchFinished { size: 1.into() }.pack()?)
        .await?;
    Ok(())
}
