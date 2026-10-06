//! Java Edition network NBT: a root tag ID followed by its payload, without a root name.

mod decode;
mod encode;
mod value;

pub use value::Nbt;
