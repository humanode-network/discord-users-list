//! List discord server members and write the output to a file
//! in the `JSONLines` format.

use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

/// An item in the output.
#[derive(Debug, serde::Serialize)]
struct OutputItem {
    /// The Discord ID of the user.
    user_id: String,
    /// The Discord username of the user.
    user_name: String,
    /// The list of all the role IDs assigned to the user.
    user_roles: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), color_eyre::eyre::Error> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let discord_token = envfury::must("DISCORD_TOKEN")?;
    let discord_server_id = envfury::must("DISCORD_SERVER_ID")?;
    let output: std::path::PathBuf = envfury::must("OUTPUT")?;

    let client = twilight_http::Client::new(discord_token);
    let mut output_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .await?;

    let mut stream = list_members::list_members(client, discord_server_id).boxed();
    while let Some(result) = stream.next().await {
        let members = result?;
        for member in members {
            let mut line = serde_json::to_string(&OutputItem {
                user_id: member.user.id.to_string(),
                user_name: member.user.name,
                user_roles: member
                    .roles
                    .into_iter()
                    .map(|role_id| role_id.to_string())
                    .collect(),
            })
            .unwrap();
            line.push('\n');
            output_file.write_all(line.as_bytes()).await?;
        }
    }

    Ok(())
}
