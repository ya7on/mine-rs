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

    /// Server description shown to clients in the server list.
    #[arg(long, default_value = "mine-rs status server")]
    pub motd: String,

    /// Maximum number of players reported in the status response.
    #[arg(long, default_value_t = 0)]
    pub max_players: u32,
}

impl Cli {
    pub fn config(&self) -> Result<Config, String> {
        let address = format!("{}:{}", self.bind, self.port);
        let addr = address
            .parse()
            .map_err(|error| format!("invalid bind address `{address}`: {error}"))?;

        Ok(Config {
            addr,
            status: StatusConfig {
                motd: self.motd.clone(),
                max_players: self.max_players,
            },
        })
    }
}

/// Settings reported to clients in the status response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusConfig {
    pub motd: String,
    pub max_players: u32,
}

/// Runtime settings for the status server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub addr: std::net::SocketAddr,
    pub status: StatusConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_default_configuration() {
        let cli = Cli::parse_from(["server"]);

        let config = cli.config().unwrap();
        assert_eq!(config.addr.to_string(), "0.0.0.0:25565");
        assert_eq!(
            config.status,
            StatusConfig {
                motd: "mine-rs status server".to_owned(),
                max_players: 0,
            }
        );
    }

    #[test]
    fn parses_custom_status_settings() {
        let cli = Cli::parse_from(["server", "--motd", "hello world", "--max-players", "42"]);

        assert_eq!(
            cli.config().unwrap().status,
            StatusConfig {
                motd: "hello world".to_owned(),
                max_players: 42,
            }
        );
    }

    #[test]
    fn rejects_an_unparsable_bind_address() {
        let cli = Cli::parse_from(["server", "--bind", "not-an-address"]);

        assert!(cli.config().is_err());
    }
}
