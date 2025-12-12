//! Export members operation.

use futures_util::StreamExt as _;
use tokio::io::AsyncWriteExt as _;

/// Params for the export members operation.
#[derive(Debug, Clone)]
pub struct Params {
    /// The discord token to use.
    pub discord_token: String,

    /// The ID of the discord server.
    pub discord_server_id: String,

    /// The path to the output file to write the data into.
    pub output: String,
}

#[allow(missing_docs, clippy::missing_docs_in_private_items)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to parse discord server id: {0}")]
    DiscordServerIdParsing(std::num::ParseIntError),

    #[error("unable to open output file: {0}")]
    OutputOpening(std::io::Error),

    #[error("discord error: {0}")]
    DiscordApi(list_members::Error),

    #[error("unable to write output: {0}")]
    OutputWriting(std::io::Error),
}

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

pub async fn run(params: Params) -> Result<(), Error> {
    let Params {
        discord_token,
        discord_server_id,
        output,
    } = params;

    let discord_server_id = discord_server_id
        .parse()
        .map_err(Error::DiscordServerIdParsing)?;

    let client = twilight_http::Client::new(discord_token);
    let mut output_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .await
        .map_err(Error::OutputOpening)?;

    let mut stream = list_members::list_members(client, discord_server_id).boxed();
    while let Some(result) = stream.next().await {
        let members = result.map_err(Error::DiscordApi)?;
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
            output_file
                .write_all(line.as_bytes())
                .await
                .map_err(Error::OutputWriting)?;
        }
    }

    Ok(())
}
