use mongodb::{Client, Database, error};
use mongodb::options::ClientOptions;
use mongodb::bson::doc;
use clap::Parser;
use crate::models::config;

pub async fn connect() -> error::Result<Database> {
    let cfg = config::Config::parse();
    let mut client_options = ClientOptions::parse(&cfg.database_url).await?;
    client_options.app_name = Some("ma".to_string());
    let client = Client::with_options(client_options)?;
    client.database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    Ok(
        client.database(&cfg.database_name)
    )
}