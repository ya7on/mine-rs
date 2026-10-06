use std::net::TcpListener;
use std::thread;

use crate::config::Config;
use crate::connection::Connection;

/// Accepts connections and serves each one on its own thread.
///
/// # Errors
///
/// Returns the I/O error from [`TcpListener::bind`]; failures on individual
/// connections are logged and never stop the accept loop.
pub fn listen(config: &Config) -> std::io::Result<()> {
    let listener = TcpListener::bind(config.addr)?;
    log::info!("listening on {}", config.addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let Ok(peer) = stream.peer_addr() else {
                    continue;
                };
                let status = config.status.clone();
                thread::spawn(move || {
                    match Connection::new(stream, status)
                        .and_then(|mut connection| connection.run())
                    {
                        Ok(()) => log::info!("{peer}: status exchange finished"),
                        Err(error) => log::info!("{peer}: closing connection: {error}"),
                    }
                });
            }
            Err(error) => log::warn!("accept failed: {error}"),
        }
    }

    Ok(())
}
