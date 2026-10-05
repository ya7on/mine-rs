use clap::Parser;

/// The `server` binary's command-line configuration.
#[derive(Parser, Debug)]
#[command(about = "A minimal Minecraft status server")]
pub struct Cli {
    /// Address to listen on.
    #[arg(long, default_value = "0.0.0.0")]
    pub bind: String,

    /// Port to listen on.
    #[arg(long, default_value_t = 25_565)]
    pub port: u16,

    /// JSON document returned to status requests.
    #[arg(
        long,
        default_value = r#"{"version":{"name":"mine-rs","protocol":777},"players":{"max":0,"online":0},"description":{"text":"mine-rs status server"}}"#
    )]
    pub status_json: String,
}

impl Cli {
    pub fn config(&self) -> Result<Config, String> {
        let address = format!("{}:{}", self.bind, self.port);
        let addr = address
            .parse()
            .map_err(|error| format!("invalid bind address `{address}`: {error}"))?;

        Ok(Config {
            addr,
            status_json: self.status_json.clone(),
        })
    }
}

/// Runtime settings for the status server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub addr: std::net::SocketAddr,
    pub status_json: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_default_configuration() {
        let cli = Cli::parse_from(["server"]);

        assert_eq!(cli.config().unwrap().addr.to_string(), "0.0.0.0:25565");
    }

    #[test]
    fn rejects_an_unparsable_bind_address() {
        let cli = Cli::parse_from(["server", "--bind", "not-an-address"]);

        assert!(cli.config().is_err());
    }
}
