use dotenv::dotenv;
use jemini::{Chat, JeminiClient as GeminiClient};
use poise::serenity_prelude::{self as serenity, UserId};
use std::env;

struct Data {
    gemini_client: GeminiClient,
    chat: Option<Chat>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// /// Displays your or another user's account creation date
// #[poise::command(slash_command, prefix_command)]
// async fn age(
//     ctx: Context<'_>,
//     #[description = "Selected user"] user: Option<serenity::User>,
// ) -> Result<(), Error> {
//     let u = user.as_ref().unwrap_or_else(|| ctx.author());
//     let response = format!("{}'s account was created at {}", u.name, u.created_at());
//     ctx.say(response).await?;
//     Ok(())
// }
#[poise::command(slash_command, prefix_command)]
async fn clear(
    ctx: Context<'_>,
    #[description = "Prompt for the chat"] prompt: String,
) -> Result<(), Error> {
    let gemini_client = &ctx.data().gemini_client;

    let timer = std::time::Instant::now();
    let chat = gemini_client.new_chat(&prompt).await?;
    println!("{}", chat.most_recent());
    println!("... in {:?}s", timer.elapsed().as_secs());

    if let Err(err) = ctx.say(chat.most_recent()).await {
        eprintln!("{}", err);
    }

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    // MTIwOTA1ODg5MTU1NjcyNDc0Ng.Gje6B7.-E8noOyXeXO2UYvq3uBDMKn4i0PQiErDTfePus
    _ = dotenv::dotenv().ok();
    let token: String = env::var("GEMINI_DISCORD_TOKEN")
        .expect("Expected a GEMINI_DISCORD_TOKEN in the environment");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![clear()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    gemini_client: GeminiClient::new().unwrap(),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await?;

    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {:#?}", data_about_bot.user);
        }
        serenity::FullEvent::Message { new_message } => {
            println!("{:#?}", new_message.mentions);
            if new_message
                .mentions
                .iter()
                .any(|u| u.id == UserId::new(1209058891556724746))
            {
                let gemini_client = &data.gemini_client;
                let timer = std::time::Instant::now();
                let chat = gemini_client.new_chat(&new_message.content).await?;
                println!("... in {:?}s", timer.elapsed().as_secs());

                new_message.reply(ctx, chat.most_recent()).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
