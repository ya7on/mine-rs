use crate::io::tcp_facade::{TcpFacadeInput, TcpFacadeOutput};
use crate::listener::Listener;
use crate::session::Session;
use crate::tcp_facade::TcpFacade;
use common::config::conf;
use common::constants::MPSC_CHANNEL_BUFFER_SIZE;
use common::error::MResult;
use common::log::{
    create_listener_span, create_session_span, create_tcp_facade_span, create_ticker_span,
    init_logging,
};
use common::tracing::{debug, info};
use std::time::Duration;
use tokio::sync::mpsc::channel;

mod io;
mod listener;
mod session;
mod tcp_facade;

#[tokio::main]
async fn ticker_thread(tcp_facade_tx: TcpFacadeInput) -> MResult<()> {
    let span = create_ticker_span();
    let _enter = span.enter();
    debug!("Ticker loop initialized");
    Ok(()) // TODO
}

#[tokio::main]
async fn listener_thread(tcp_facade_tx: TcpFacadeInput) -> MResult<()> {
    let span = create_listener_span();
    let _enter = span.enter();
    let c = conf();
    info!("Starting Minecraft server on {}:{}", c.host, c.port);
    let tcp_listener = Listener::new(format!("{}:{}", c.host, c.port)).await?;
    loop {
        let (session, write_socket) = tcp_listener.accept().await?;
        tcp_facade_tx
            .new_session(session.session_id, write_socket)
            .await?;
        let for_session_tx = tcp_facade_tx.clone();
        std::thread::spawn(move || session_thread(session, for_session_tx));
    }
}

#[tokio::main]
async fn session_thread(mut session: Session, tcp_facade_tx: TcpFacadeInput) -> MResult<()> {
    let span = create_session_span(session.session_id.to_string());
    let _enter = span.enter();
    info!("Accepted new connection");
    session.run(tcp_facade_tx).await?;
    Ok(())
}

#[tokio::main]
async fn tcp_writer_thread(input: TcpFacadeOutput) -> MResult<()> {
    let span = create_tcp_facade_span();
    let _enter = span.enter();

    let mut facade = TcpFacade::new(input);

    facade.run().await
}

fn main() {
    init_logging();

    let (tx, rx) = channel(MPSC_CHANNEL_BUFFER_SIZE);

    let for_listener_tx = TcpFacadeInput::from(tx.clone());
    let for_ticket_tx = TcpFacadeInput::from(tx);

    vec![
        std::thread::spawn(|| listener_thread(for_listener_tx)),
        std::thread::spawn(|| ticker_thread(for_ticket_tx)),
        std::thread::spawn(|| tcp_writer_thread(TcpFacadeOutput::from(rx))),
    ];

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
