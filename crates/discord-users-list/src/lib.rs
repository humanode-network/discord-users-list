//! Implemntation of the discord users listing logic.

use twilight_model::id::{marker::GuildMarker, Id};

/// An error that can occur while listing users.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error while reading the data.
    #[error("request: {0}")]
    Request(twilight_http::Error),
    /// Error while reading the data.
    #[error("response: {0}")]
    Response(twilight_http::response::DeserializeBodyError),
}

/// List the discord users.
pub fn list_users(
    client: twilight_http::Client,
    guild_id: Id<GuildMarker>,
) -> impl futures_core::Stream<Item = Result<Vec<twilight_model::guild::Member>, Error>> {
    async_stream::try_stream! {
        let mut after = None;

        loop {
            let mut request = client.guild_members(guild_id).limit(1000).unwrap();
            if let Some(after) = after.take() {
                request = request.after(after);
            }
            let response = request.await.map_err(Error::Request)?;
            debug_response(&response);

            let models = response.models().await.map_err(Error::Response)?;

            let last_model = match models.last() {
                Some(val) => val,
                None => break,
            };

            after = Some(last_model.user.id);

            yield models;
        }
    }
}

/// Debug print the response.
fn debug_response<T: std::fmt::Debug>(res: &twilight_http::Response<T>) {
    let headers: std::collections::HashMap<&str, &[u8]> = res.headers().collect();

    let rate_limit =
        String::from_utf8_lossy(headers.get("x-ratelimit-remaining").unwrap_or(&&b""[..]))
            .into_owned();

    tracing::debug!(
        message = "Got response",
        %rate_limit
    );
}
