//! Minecraft data types
//! <a href="https://wiki.vg/Data_types">Wiki.vg</a>

pub mod types;

pub use types::MinecraftType;

pub use types::boolean::MinecraftBoolean;
pub use types::byte::MinecraftByte;
pub use types::int::MinecraftInt;
pub use types::long::MinecraftLong;
pub use types::short::MinecraftShort;
pub use types::string::MinecraftString;
pub use types::unsigned_byte::MinecraftUnsignedByte;
pub use types::unsigned_short::MinecraftUnsignedShort;
pub use types::uuid::MinecraftUUID;
pub use types::varint::MinecraftVarInt;
