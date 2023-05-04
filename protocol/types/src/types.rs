use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;

pub mod bitset;
pub mod boolean;
pub mod byte;
pub mod int;
pub mod long;
pub mod short;
pub mod string;
pub mod unsigned_byte;
pub mod unsigned_short;
pub mod uuid;
pub mod varint;

pub const SEGMENT_BITS: u8 = 0b0111_1111;
pub const CONTINUE_BIT: u8 = 0b1000_0000;

/// Base trait for Minecraft data type
#[async_trait]
pub trait MinecraftType
where
    Self: Sized,
{
    /// Converts [Read Buffer](common::io::Buffer) to it's structure
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self>;
    /// Converts structure to vectored bytes
    async fn parse_to(&self) -> MResult<Vec<u8>>;
}
