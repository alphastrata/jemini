# Discord Bot Example

This is a Discord bot example that uses the [Poise crate](https://docs.rs/poise) for command handling and the [Jemini API](https://github.com/jemini/jemini) for chat generation.

## Prerequisites

- A Discord bot account with a token from Discord.
- A Gemini Ai Studio API key from Google.

## Setting Up the Discord Bot

1. Create a new Discord bot account on the [Discord Developer Portal](https://discord.com/developers/applications).
2. Navigate to the "Bot" tab and create a new bot.
3. Copy the bot token and keep it secure.

## Setting Up the Jemini API Key

1. Follow the instructions in the [Jemini README](../../README.md) to obtain a Gemini/Google AI Studio API key.
2. Store the API key in a `.env` file at the root of your project:
   > .env file

```
   GEMINI_API_KEY=your_jemini_api_key_here
   GEMINI_DISCORD_BOT=your_discord_bot_token_here
```

## Running the Bot

1. Clone this repository or copy the necessary files into your project.
2. I assume you have Rust already...
3. Navigate to the project directory and run the following command to install dependencies:
4. Run the bot:

```sh
cargo run --release
```

5. Invite the bot to your Discord server using the OAuth2 URL generated in the Discord Developer Portal.

6. Once the bot is running, you can start a chat by mentioning the bot with the command `@GeminiBot`.

## What can it do?

- `@GeminiBot`: Starts a new chat with the Jemini API, or you can just start typing. Be careful in a server -- should you choose to do that -- as it'll just assume everything is directed at it.
- Uploading a photo with text will be interpreted by the GeminiPro Vision model.

### todos:

- `/clear` will wipe chat history.
