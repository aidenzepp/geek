mod auth;
mod data;
mod form;
mod geek;

#[tokio::main]
async fn main() -> form::Result<()> {
    return geek::Geek::run().await;
}
