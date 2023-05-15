# discord-users-list

A sample app that loads the list of members from a Discord server and their
roles.

Useful for integration with [BotBasher](https://botbasher.humanode.io).

## Requirements

- Rust
- Bash-like shell (optional, for passing the env vars, you can use other methods as well)

## Usage

1. Clone repo.

2. Write settings.

   ```shell
   cat <<'EOF' > .env1
   export RUST_LOG="info,discord_users_list=debug"
   export DISCORD_TOKEN="<token>"
   export DISCORD_SERVER_ID="<server id>"
   export OUTPUT="users.jsonlines"
   EOF
   ```

3. Load settings into current shell.

   ```shell
   source .env
   ```

4. Run the app.

   ```shell
   cargo run
   ```

5. Inspect the users list and the execution log.

   Please report issues you encounter with this sample app!
