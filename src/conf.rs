use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Parameters {
    #[clap(default_value = "127.0.0.1", env = "MINERS_HOST")]
    pub host: String,

    #[clap(default_value = "25565", env = "MINERS_PORT")]
    pub port: u16,
}

pub fn conf() -> &'static Parameters {
    lazy_static! {
        static ref PARAMS: Parameters = Parameters::parse();
    };
    &PARAMS
}
