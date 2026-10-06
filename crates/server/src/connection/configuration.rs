use std::collections::BTreeMap;

use mclib::nbt::Nbt;
use mclib::packets::configuration::{
    ClientInformation, FeatureFlags, FinishConfiguration, KnownPack, KnownPacks, RegistryData,
    RegistryEntry, RegistryTags, Tag, UpdateTags,
};
use mclib::{MCString, MCType, PacketFrame, ProtocolError};

use crate::ConnectionError;
use crate::connection::Connection;

/// Negotiates a fixed, minimal vanilla registry set for the 26.3 client.
pub async fn run(connection: &mut Connection) -> Result<bool, ConnectionError> {
    let known = KnownPacks {
        packs: vec![KnownPack {
            namespace: "minecraft".into(),
            id: "core".into(),
            version: "26.3".into(),
        }]
        .into(),
    };
    connection.write_frame(15, known.pack()?).await?;
    let frame = receive(connection, 7).await?;
    let mut body = frame.body.as_slice();
    let selected = KnownPacks::unpack(&mut body)?;
    if !body.is_empty() {
        return Err(ProtocolError::InvalidData.into());
    }
    if selected != known {
        connection
            .write_frame(
                2,
                Nbt::compound(vec![(
                    "text",
                    Nbt::string("This server requires the vanilla 26.3 core pack"),
                )])
                .encode_network()?,
            )
            .await?;
        return Ok(false);
    }

    let registries: BTreeMap<String, Vec<String>> =
        serde_json::from_str(include_str!("../../data/registries-777.json"))
            .map_err(|_| ProtocolError::InvalidData)?;
    // Registry entry order fixes numeric IDs; the selected entries use ID 0.
    for (id, entries) in registries {
        connection
            .write_frame(
                7,
                RegistryData {
                    registry_id: id.into(),
                    entries: entries
                        .into_iter()
                        .map(|id| RegistryEntry {
                            id: id.into(),
                            data: None,
                        })
                        .collect::<Vec<_>>()
                        .into(),
                }
                .pack()?,
            )
            .await?;
    }

    // Feature Flags: the vanilla feature set. Update Tags: the one timeline
    // dependency of the selected overworld dimension, referencing entry ID 0.
    connection
        .write_frame(
            13,
            FeatureFlags {
                flags: vec!["minecraft:vanilla".into()].into(),
            }
            .pack()?,
        )
        .await?;
    connection
        .write_frame(
            14,
            UpdateTags {
                registries: vec![RegistryTags {
                    registry_id: "minecraft:timeline".into(),
                    tags: vec![Tag {
                        name: "minecraft:in_overworld".into(),
                        entries: vec![0.into()].into(),
                    }]
                    .into(),
                }]
                .into(),
            }
            .pack()?,
        )
        .await?;
    connection
        .write_frame(3, FinishConfiguration.pack()?)
        .await?;
    let frame = receive(connection, 3).await?;
    if !frame.body.is_empty() {
        return Err(ProtocolError::InvalidData.into());
    }
    log::info!("Configuration acknowledged by client");
    Ok(true)
}

async fn receive(
    connection: &mut Connection,
    expected: i32,
) -> Result<PacketFrame, ConnectionError> {
    loop {
        let frame = connection.read_frame().await?;
        if frame.packet_id.0 == expected {
            return Ok(frame);
        }
        let mut body = frame.body.as_slice();
        match frame.packet_id.0 {
            0 => {
                ClientInformation::unpack(&mut body)?;
            }
            2 => {
                // Plugin payloads have no effect on this vanilla connection.
                MCString::<32767>::unpack(&mut body)?;
                if body.len() > 1_048_576 {
                    return Err(ProtocolError::Overflow.into());
                }
                continue;
            }
            id => {
                return Err(ConnectionError::UnexpectedPacket {
                    packet_id: id,
                    expected: "configuration response",
                });
            }
        }
        if !body.is_empty() {
            return Err(ProtocolError::InvalidData.into());
        }
    }
}
