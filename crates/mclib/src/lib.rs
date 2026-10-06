//! Shared Minecraft protocol types and codecs.

pub mod codec;
pub mod error;
pub mod nbt;
pub mod packet_frame;
pub mod packets;
pub mod types;

pub use codec::MCType;
pub use error::ProtocolError;
pub use packet_frame::PacketFrame;
pub use types::{
    bitset::MCBitSet, boolean::MCBoolean, byte::MCByte, byte_array::MCByteArray, double::MCDouble,
    float::MCFloat, int::MCInt, long::MCLong, position::MCPosition,
    prefixed_array::MCPrefixedArray, short::MCShort, string::MCString,
    unsigned_byte::MCUnsignedByte, unsigned_short::MCUnsignedShort, uuid::MCUuid,
    var_int::MCVarInt, var_long::MCVarLong,
};
