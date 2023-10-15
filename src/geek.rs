// clap...
use clap::Parser;

// crate...
use crate::form;
use crate::auth;
use crate::data;

#[derive(Debug, Parser)]
#[command(name = "MintedGeek CLI")]
#[command(author = "MintedGeek")]
#[command(version = "1.0")]
pub struct Geek {
    /// Represents the 
    #[command(subcommand)]
    section: form::Section,
}

impl Geek {
    pub async fn run() -> form::Result<()> {
        let client = reqwest::Client::new();

        return match Geek::parse().section {
            form::Section::Auth(command) => Self::handle_auth(&client, command).await,
            form::Section::Data(command) => Self::handle_data(&client, command).await,
        };
    }

    async fn handle_auth(client: &reqwest::Client, command: auth::Command) -> form::Result<()> {
        return auth::Command::handle(&client, command).await;
    }

    async fn handle_data(client: &reqwest::Client, command: data::Command) -> form::Result<()> {
        return data::Command::handle(&client, command).await;
    }
}
