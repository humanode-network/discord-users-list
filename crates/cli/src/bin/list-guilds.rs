//! List discord guilds (aka servers) that a given discord token has access to.

#[tokio::main]
async fn main() -> Result<(), color_eyre::eyre::Error> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let discord_token = envfury::must("DISCORD_TOKEN")?;

    let client = twilight_http::Client::new(discord_token);

    let guilds_response = client.current_user_guilds().await?;
    let guilds = guilds_response.models().await?;

    for guild in guilds {
        println!("{} {}", guild.id, guild.name);
    }

    Ok(())
}
