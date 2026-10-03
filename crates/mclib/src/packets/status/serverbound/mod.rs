//! Status packets sent by the client to the server.

mod ping_request;
mod status_request;

pub use ping_request::PingRequest;
pub use status_request::StatusRequest;
