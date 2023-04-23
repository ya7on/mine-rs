use crate::MinecraftPacket;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;
use serde::Serialize;
use types::{MinecraftString, MinecraftType};

#[derive(Serialize, Debug)]
pub struct Description {
    pub text: String,
}

#[derive(Serialize, Debug)]
pub struct Struct {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Debug)]
pub struct Players {
    pub max: usize,
    pub online: usize,
}

#[derive(Serialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: usize,
}

#[derive(Serialize, Debug)]
pub struct StatusResponsePacket {
    pub version: Version,
    pub players: Players,
    pub description: Description,
}

#[async_trait]
impl MinecraftPacket for StatusResponsePacket {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        unimplemented!()
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let json = serde_json::to_string(self).unwrap();
        MinecraftString(json).parse_to().await
    }
}
