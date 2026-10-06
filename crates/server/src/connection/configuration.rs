use std::collections::BTreeMap;

use mclib::nbt::Nbt;
use mclib::packets::configuration::{
    ClientInformation, FeatureFlags, FinishConfiguration, KnownPack, KnownPacks, RegistryData,
    RegistryEntry, RegistryTags, Tag, UpdateTags,
};
use mclib::{MCString, MCType, PacketFrame, ProtocolError};

use crate::ConnectionError;
use crate::connection::Connection;

/// Negotiates a complete vanilla registry set for the 26.3 client.
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
    // Registry entry order fixes numeric IDs used by the exported dynamic tags.
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

    connection
        .write_frame(
            13,
            FeatureFlags {
                flags: vec!["minecraft:vanilla".into()].into(),
            }
            .pack()?,
        )
        .await?;
    connection.write_frame(14, vanilla_tags()?.pack()?).await?;
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

// Dynamic IDs match our Registry Data order; static IDs come from Mojang's report.
fn vanilla_tags() -> Result<UpdateTags, ProtocolError> {
    let data: BTreeMap<String, BTreeMap<String, Vec<i32>>> =
        serde_json::from_str(include_str!("../../data/tags-777.json"))
            .map_err(|_| ProtocolError::InvalidData)?;
    Ok(UpdateTags {
        registries: data
            .into_iter()
            .map(|(registry_id, tags)| RegistryTags {
                registry_id: registry_id.into(),
                tags: tags
                    .into_iter()
                    .map(|(name, entries)| Tag {
                        name: name.into(),
                        entries: entries
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<_>>()
                            .into(),
                    })
                    .collect::<Vec<_>>()
                    .into(),
            })
            .collect::<Vec<_>>()
            .into(),
    })
}
