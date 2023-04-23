use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;

pub mod boolean;
pub mod byte;
pub mod long;
pub mod string;
pub mod unsigned_byte;
pub mod unsigned_short;
pub mod uuid;
pub mod varint;

pub const SEGMENT_BITS: u8 = 0b0111_1111;
pub const CONTINUE_BIT: u8 = 0b1000_0000;

#[async_trait]
pub trait MinecraftType
where
    Self: Sized,
{
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self>;
    async fn parse_to(&self) -> MResult<Vec<u8>>;
}
