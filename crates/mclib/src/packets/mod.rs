//! Minecraft packet bodies grouped by protocol state.
//!
//! Packet IDs and frame lengths are transport concerns and are intentionally
//! not represented here.

pub mod handshaking;
pub mod status;
