use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Parameters {
    #[clap(default_value = "127.0.0.1", env = "MINERS_HOST")]
    pub host: String,

    #[clap(default_value = "25565", env = "MINERS_PORT")]
    pub port: u16,

    #[clap(default_value = "A Minecraft server", env = "MINERS_MOTD")]
    pub motd: String,

    #[clap(default_value = "Mine.rs", env = "MINERS_APP_NAME")]
    pub app_name: String,

    #[clap(default_value = "25", env = "MINERS_MAX_PLAYERS")]
    pub max_players: usize,
}

pub fn conf() -> &'static Parameters {
    lazy_static! {
        static ref PARAMS: Parameters = Parameters::parse();
    };
    &PARAMS
}
