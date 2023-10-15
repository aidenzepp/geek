
pub type Object = String;

pub type Record = String;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, clap::Subcommand)]
pub enum Section {
    /// ...
    #[command(subcommand)]
    Auth(crate::auth::Command),

    /// ...
    #[command(subcommand)]
    Data(crate::data::Command),

}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Arguments(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}