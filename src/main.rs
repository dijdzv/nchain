use anyhow::Result;
use axum::{routing::get, Router};
use settings::Settings;

mod settings;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let settings = Settings::new()?;
    let server_address = format!("{}:{}", settings.server_address, settings.server_port);

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind(server_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
