use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Conf {
    #[arg(long, default_value = "0.0.0.0", env = "M_HOST")]
    pub host: String,

    #[arg(long, default_value = "25565", env = "M_PORT")]
    pub port: u16,
}

pub fn conf() -> &'static Conf {
    lazy_static! {
        static ref OPT: Conf = Conf::parse();
    };
    &OPT
}
