use tokio::net::TcpListener;

use crate::config::Config;
use crate::connection::Connection;

/// Accepts connections and serves each one in its own Tokio task.
///
/// # Errors
///
/// Returns the I/O error from binding; individual connection failures are logged.
pub async fn listen(config: &Config) -> std::io::Result<()> {
    let listener = TcpListener::bind(config.addr).await?;
    log::info!("listening on {}", config.addr);

    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                let status = config.status.clone();
                tokio::spawn(async move {
                    match Connection::new(stream, status).run().await {
                        Ok(()) => log::info!("{peer}: status exchange finished"),
                        Err(error) => log::info!("{peer}: closing connection: {error}"),
                    }
                });
            }
            Err(error) => log::warn!("accept failed: {error}"),
        }
    }
}
