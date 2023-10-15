// clap...
use clap::Parser;

// crate...
use crate::form::Section;
use crate::auth;
use crate::data;

#[derive(Debug, clap::Parser)]
#[command(name = "MintedGeek CLI")]
#[command(author = "MintedGeek")]
#[command(version = "1.0")]
pub struct Geek {
    /// Represents the 
    #[command(subcommand)]
    section: Section,
}

impl Geek {
    pub fn run() {
        match Geek::parse().section {
            Section::Auth(command) => {
                let _ = auth::Command::handle(command);
            },
            Section::Data(command) => {
                let _ = data::Command::handle(command);
            },
        }
    }
}