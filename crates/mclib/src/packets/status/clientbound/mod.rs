//! Status packets sent by the server to the client.

mod pong_response;
mod status_response;

pub use pong_response::PongResponse;
pub use status_response::StatusResponse;
