use clap::Parser;

use server::config::Cli;
use server::listener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let config = match cli.config() {
        Ok(config) => config,
        Err(message) => {
            eprintln!("error: {message}");
            std::process::exit(2);
        }
    };

    if let Err(error) = listener::listen(&config).await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
