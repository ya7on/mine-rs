use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;
use types::{MinecraftType, MinecraftVarInt};

pub mod handshake;
pub mod from_client {
    pub mod status {
        pub mod ping;
    }
    pub mod login {
        pub mod login_start;
    }
}
pub mod from_server {
    pub mod status {
        pub mod status_response;
    }
    pub mod login {
        pub mod login_success;
    }
}

#[async_trait]
pub trait MinecraftPacket
where
    Self: Sized,
{
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self>;
    async fn parse_to(&self) -> MResult<Vec<u8>>;
    async fn to_packet(&self, packet_id: MinecraftVarInt) -> MResult<Vec<u8>> {
        let mut final_bytes = Vec::new();
        let payload = self.parse_to().await?;
        let package_len =
            MinecraftVarInt((payload.len() + packet_id.parse_to().await?.len()) as i32);
        final_bytes.extend_from_slice(&package_len.parse_to().await?);
        final_bytes.extend_from_slice(&packet_id.parse_to().await?);
        final_bytes.extend_from_slice(&payload);
        Ok(final_bytes)
    }
}
