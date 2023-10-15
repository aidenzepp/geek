#![allow(unused_variables)]
#![allow(dead_code)]

// crate...
use crate::form::Result;

#[derive(Debug, clap::Subcommand)]
pub enum Command {}

impl Command {
    pub async fn handle(client: &reqwest::Client, command: Command) -> Result<()> {
        return Ok(());
    }
}