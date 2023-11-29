#[macro_use]
extern crate tracing;

use crate::conf::conf;
use crate::server::app::{MinersApp, MinersParameters};
use tracing::Level;
use tracing_subscriber::EnvFilter;

mod conf;
mod registry;
mod server;

fn main() {
    let c = conf();

    let env_filter = EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
    let root_span = span!(Level::ERROR, "main");
    let _enter = root_span.enter();

    MinersApp::new(MinersParameters {
        host: c.host.clone(),
        port: c.port,
    })
    .run()
}
