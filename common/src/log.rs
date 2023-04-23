use tracing::{Level, Span};
use tracing_subscriber::EnvFilter;

const TICKER_SPAN_NAME: &str = "Ticker";
const LISTENER_SPAN_NAME: &str = "TCP Listener";
const SESSION_SPAN_NAME: &str = "Session";
const TCP_FACADE_SPAN_NAME: &str = "TCP Writer";

/// Initialize project logger
pub fn init_logging() {
    let env_filter = EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

pub fn create_ticker_span() -> Span {
    span!(Level::DEBUG, TICKER_SPAN_NAME)
}

pub fn create_listener_span() -> Span {
    span!(Level::DEBUG, LISTENER_SPAN_NAME)
}

pub fn create_session_span(session_id: String) -> Span {
    span!(Level::DEBUG, SESSION_SPAN_NAME, session_id)
}

pub fn create_tcp_facade_span() -> Span {
    span!(Level::DEBUG, TCP_FACADE_SPAN_NAME)
}
