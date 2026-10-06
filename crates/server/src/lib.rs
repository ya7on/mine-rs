//! A minimal Minecraft server that only serves the status protocol.

pub mod config;
pub mod connection;
pub mod listener;

pub mod error;

pub use error::ConnectionError;
