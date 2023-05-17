//! Prints the bot application resource in the JSON format to stdout.

#[tokio::main]
async fn main() -> Result<(), color_eyre::eyre::Error> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let discord_token = envfury::must("DISCORD_TOKEN")?;

    let client = twilight_http::Client::new(discord_token);

    let app_response = client.current_user_application().await?;
    let app = app_response.model().await?;

    println!("{}", serde_json::to_string_pretty(&app).unwrap());

    Ok(())
}
