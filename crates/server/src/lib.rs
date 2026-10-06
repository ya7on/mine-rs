//! A minimal Minecraft server supporting Status and offline Login.

pub mod config;
pub mod connection;
pub mod listener;

pub mod error;

pub use error::ConnectionError;
