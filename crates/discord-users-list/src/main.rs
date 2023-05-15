//! List discord users.

#[tokio::main]
async fn main() -> Result<(), color_eyre::eyre::Error> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let _discord_token: String = envfury::must("DISCORD_TOKEN")?;

    Ok(())
}
