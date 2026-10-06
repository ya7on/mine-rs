use mclib::packets::play::clientbound::{Login, SynchronizePlayerPosition};
use mclib::packets::play::serverbound::ConfirmTeleportation;
use mclib::{MCType, ProtocolError};

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
