
pub type Object = String;

pub type Record = String;

#[derive(Debug, clap::Subcommand)]
pub enum Section {
    /// ...
    #[command(subcommand)]
    Auth(crate::auth::Command),

    /// ...
    #[command(subcommand)]
    Data(crate::data::Command),

}