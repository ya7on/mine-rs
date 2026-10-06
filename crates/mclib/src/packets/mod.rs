//! Minecraft packet bodies grouped by protocol state.
//!
//! Packet IDs and frame lengths are transport concerns and are intentionally
//! not represented here.

pub mod configuration;
pub mod handshaking;
pub mod login;
pub mod status;
