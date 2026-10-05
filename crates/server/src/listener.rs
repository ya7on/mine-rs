use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::config::{Config, StatusConfig};
use crate::connection::Connection;

/// Accepts connections and serves each one on its own thread.
pub fn listen(config: Config) -> std::io::Result<()> {
    let listener = TcpListener::bind(config.addr)?;
    log::info!("listening on {}", config.addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => spawn_handler(stream, config.status.clone()),
            Err(error) => log::warn!("accept failed: {error}"),
        }
    }

    Ok(())
}

fn spawn_handler(stream: TcpStream, status: StatusConfig) {
    let Ok(peer) = stream.peer_addr() else {
        return;
    };

    thread::spawn(move || {
        match Connection::new(stream, status).and_then(|mut connection| connection.run()) {
            Ok(()) => log::info!("{peer}: status exchange finished"),
            Err(error) => log::info!("{peer}: closing connection: {error}"),
        }
    });
}
