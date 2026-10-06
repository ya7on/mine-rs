use mclib::packets::play::clientbound::{
    ChunkBatchFinished, ChunkDataAndUpdateLight, EmptyChunkSection, GameEvent, Heightmap, Login,
    SetCenterChunk, SetDefaultSpawnPosition, SynchronizePlayerPosition,
};
use mclib::packets::play::serverbound::ConfirmTeleportation;
use mclib::{MCBitSet, MCPosition, MCType, PacketFrame, ProtocolError};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::ConnectionError;
use crate::connection::{Connection, READ_TIMEOUT};

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(10);
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(15);

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
    let (mut reader, mut writer) = connection.stream.split();
    serve(&mut reader, &mut writer, &position).await
}

async fn serve(
    reader: &mut (impl AsyncRead + Unpin),
    writer: &mut (impl AsyncWrite + Unpin),
    position: &SynchronizePlayerPosition,
) -> Result<(), ConnectionError> {
    // Both borrowed halves stay in this task. Pin each full-frame read across
    // timer events so a partial frame is never cancelled and restarted.
    let mut id = 1_i64;
    PacketFrame::new(0x2D, KeepAlive { id: id.into() }.pack()?)
        .write(&mut *writer)
        .await?;
    let mut pending = Some(id);
    let mut sent_at = Instant::now();
    let teleport_deadline = sent_at + RESPONSE_TIMEOUT;
    let mut teleport_confirmed = false;
    loop {
        let read = timeout(READ_TIMEOUT, PacketFrame::read(&mut *reader));
        tokio::pin!(read);
        let frame = loop {
            let mut deadline = sent_at
                + if pending.is_some() {
                    RESPONSE_TIMEOUT
                } else {
                    KEEP_ALIVE_INTERVAL
                };
            if !teleport_confirmed {
                deadline = deadline.min(teleport_deadline);
            }
            tokio::select! {
                biased;
                () = sleep_until(deadline) => {
                    if !teleport_confirmed && Instant::now() >= teleport_deadline {
                        return Err(Error::new(ErrorKind::TimedOut, "teleport acknowledgement timed out").into());
                    }
                    if pending.is_some() {
                        return Err(Error::new(ErrorKind::TimedOut, "keep alive response timed out").into());
                    }
                    id = id.checked_add(1).ok_or(ProtocolError::Overflow)?;
                    PacketFrame::new(0x2D, KeepAlive { id: id.into() }.pack()?)
                        .write(&mut *writer).await?;
                    pending = Some(id);
                    sent_at = Instant::now();
                }
                result = &mut read => {
                    break result.map_err(|_| Error::new(ErrorKind::TimedOut, "frame read timed out"))??;
                }
            }
        };
        let mut body = frame.body.as_slice();
        match frame.packet_id.0 {
            0 => {
                let confirmation = ConfirmTeleportation::unpack(&mut body)?;
                if teleport_confirmed
                    || confirmation.teleport_id != position.teleport_id
                    || confirmation.x != position.x
                    || confirmation.y != position.y
                    || confirmation.z != position.z
                    || confirmation.yaw != position.yaw
                    || confirmation.pitch != position.pitch
                {
                    return Err(ProtocolError::InvalidData.into());
                }
                teleport_confirmed = true;
                log::info!("Play teleport acknowledged by client");
            }
            0x1C => {
                let reply = KeepAlive::unpack(&mut body)?;
                if pending != Some(reply.id.0) {
                    return Err(ProtocolError::InvalidData.into());
                }
                pending = None;
            }
            // No movement, interactions or world simulation in this demo.
            _ => continue,
        }
        if !body.is_empty() {
            return Err(ProtocolError::InvalidData.into());
        }
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
use std::io::{Error, ErrorKind};
use std::time::Duration;

use tokio::time::{Instant, sleep_until, timeout};

use mclib::packets::play::KeepAlive;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncWriteExt, DuplexStream, duplex, split};
    use tokio::task::JoinHandle;
    use tokio::time::advance;

    async fn connected(confirm: bool) -> (DuplexStream, JoinHandle<Result<(), ConnectionError>>) {
        let (client, server) = duplex(1024);
        let task = tokio::spawn(async move {
            let (mut reader, mut writer) = split(server);
            serve(
                &mut reader,
                &mut writer,
                &SynchronizePlayerPosition {
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
                },
            )
            .await
        });
        let mut client = client;
        if confirm {
            PacketFrame::new(
                0,
                ConfirmTeleportation {
                    teleport_id: 1.into(),
                    x: 8.0.into(),
                    y: 100.0.into(),
                    z: 8.0.into(),
                    yaw: 0.0.into(),
                    pitch: 0.0.into(),
                }
                .pack()
                .unwrap(),
            )
            .write(&mut client)
            .await
            .unwrap();
        }
        (client, task)
    }

    async fn receive(client: &mut DuplexStream) -> KeepAlive {
        let frame = PacketFrame::read(client).await.unwrap();
        assert_eq!(frame.packet_id.0, 0x2D);
        KeepAlive::unpack(&mut frame.body.as_slice()).unwrap()
    }

    async fn echo(client: &mut DuplexStream, packet: KeepAlive) {
        PacketFrame::new(0x1C, packet.pack().unwrap())
            .write(client)
            .await
            .unwrap();
        tokio::task::yield_now().await;
    }

    #[tokio::test(start_paused = true)]
    async fn timer_survives_partial_read() {
        let (mut client, task) = connected(true).await;
        let first = receive(&mut client).await;
        echo(&mut client, first.clone()).await;
        client.write_all(&[3]).await.unwrap();
        tokio::task::yield_now().await;
        advance(KEEP_ALIVE_INTERVAL).await;
        let second = receive(&mut client).await;
        assert_ne!(second.id, first.id);
        client.write_all(&[0x0D, 0, 0]).await.unwrap();
        echo(&mut client, second.clone()).await;
        advance(KEEP_ALIVE_INTERVAL).await;
        let third = receive(&mut client).await;
        assert_ne!(third.id, second.id);
        assert!(!task.is_finished());
        task.abort();
    }

    #[tokio::test(start_paused = true)]
    async fn unrelated_packets_do_not_reset_reply_deadline() {
        let (mut client, task) = connected(true).await;
        receive(&mut client).await;
        for _ in 0..3 {
            advance(Duration::from_secs(4)).await;
            PacketFrame::new(0x0D, Vec::new())
                .write(&mut client)
                .await
                .unwrap();
            tokio::task::yield_now().await;
        }
        advance(Duration::from_secs(4)).await;
        assert!(
            matches!(task.await.unwrap(), Err(ConnectionError::Io(error)) if error.kind() == ErrorKind::TimedOut)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn rejects_trailing_keep_alive_bytes() {
        let (mut client, task) = connected(true).await;
        let packet = receive(&mut client).await;
        let mut body = packet.pack().unwrap();
        body.push(0);
        PacketFrame::new(0x1C, body)
            .write(&mut client)
            .await
            .unwrap();
        assert!(matches!(
            task.await.unwrap(),
            Err(ConnectionError::Protocol(ProtocolError::InvalidData))
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn teleport_deadline_survives_keep_alive_replies() {
        let (mut client, task) = connected(false).await;
        let first = receive(&mut client).await;
        echo(&mut client, first).await;
        advance(KEEP_ALIVE_INTERVAL).await;
        let second = receive(&mut client).await;
        echo(&mut client, second).await;
        advance(Duration::from_secs(6)).await;
        assert!(
            matches!(task.await.unwrap(), Err(ConnectionError::Io(error)) if error.kind() == ErrorKind::TimedOut)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn rejects_wrong_teleport_position() {
        let (mut client, task) = connected(false).await;
        receive(&mut client).await;
        PacketFrame::new(
            0,
            ConfirmTeleportation {
                teleport_id: 1.into(),
                x: 8.0.into(),
                y: 99.0.into(),
                z: 8.0.into(),
                yaw: 0.0.into(),
                pitch: 0.0.into(),
            }
            .pack()
            .unwrap(),
        )
        .write(&mut client)
        .await
        .unwrap();
        assert!(matches!(
            task.await.unwrap(),
            Err(ConnectionError::Protocol(ProtocolError::InvalidData))
        ));
    }
}
