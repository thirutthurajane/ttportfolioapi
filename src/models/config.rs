#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env)]
    pub database_name: String,
    #[clap(long, env)]
    pub secret: String,
    #[clap(long, env)]
    pub host: String,
    #[clap(long, env)]
    pub port: String
}