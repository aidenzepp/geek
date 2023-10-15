#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, clap::Subcommand)]
pub enum Command {}

impl Command {
    pub fn handle(command: Command) -> Result<(), std::io::Error> {
        return Ok(());
    }
}