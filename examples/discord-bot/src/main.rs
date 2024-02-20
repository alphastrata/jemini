use image::ImageFormat;
use jemini::{Chat, ImageData, JeminiClient as GeminiClient};
use poise::serenity_prelude::{self as serenity, UserId};
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

const BOT_ID: u64 = 1209058891556724746;

struct Data {
    gemini_client: GeminiClient,
    chats: Arc<Mutex<HashMap<UserId, Chat>>>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'c> = poise::Context<'c, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let u = ctx.author();

    let data = ctx.data();
    if data.chats.lock().await.remove(&u.id).is_some() {
        _ = ctx.reply("Cleared!").await;
    }

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    _ = dotenv::dotenv().ok();
    let token: String = env::var("GEMINI_DISCORD_TOKEN")
        .expect("Expected a GEMINI_DISCORD_TOKEN in the environment");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILDS;

    let chats = HashMap::new();
    let m_chats = Arc::new(Mutex::new(chats));

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
                    gemini_client: GeminiClient::new()?,
                    chats: m_chats,
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

async fn event_handler<'c>(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    let gemini_client = &data.gemini_client;

    match event {
        // serenity::FullEvent::Ready { data_about_bot, .. } => {
        // println!("Logged in as {:#?}", data_about_bot.user);
        // }
        serenity::FullEvent::Message { new_message } => {
            //

            // TODO: Handle chat existing

            if new_message.author.id != UserId::new(BOT_ID) {
                // Handle img:
                if let Some(attachment) = new_message.attachments.first() {
                    if let Ok(content) = attachment.download().await {
                        println!("filename:{}", attachment.filename);
                        println!("img_size: {}b", content.len());
                        let format = match PathBuf::from(&attachment.filename).extension() {
                            Some(ext) => match ext.to_str() {
                                Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
                                Some("png") => ImageFormat::Png,
                                Some("gif") => ImageFormat::Gif,
                                Some("webp") => ImageFormat::WebP,
                                _ => {
                                    eprintln!("Unsupported image format");
                                    return Ok(());
                                }
                            },
                            None => {
                                eprintln!("No file extension found");
                                return Ok(());
                            }
                        };
                        println!("{}", format.to_mime_type());
                        if let Ok(image_data) = ImageData::from_bytes(content.into(), format) {
                            if let Ok(response) = gemini_client
                                .text_and_image(&new_message.content, image_data)
                                .await
                            {
                                _ = new_message
                                    .reply(
                                        ctx,
                                        response.most_recent().unwrap_or_else(|| {
                                            "Error receiving a response from Gemini"
                                        }),
                                    )
                                    .await?;
                            }
                        }
                    }

                    // Be sure to break as these will get loooooong...
                    return Ok(());
                }

                // Handle chat
                if new_message
                    .mentions
                    .iter()
                    .any(|u| u.id == UserId::new(BOT_ID))
                {
                    let timer = std::time::Instant::now();
                    let chat = gemini_client.new_chat(&new_message.content).await?;
                    println!("... in {:?}s", timer.elapsed().as_secs());

                    // Check if chat exists for the user:
                    let u = &new_message.author;
                    let mut chats = data.chats.lock().await;

                    new_message.reply(ctx, chat.most_recent()).await?;

                    if let Some(existing_chat) = chats.get_mut(&u.id) {
                        // If chat exists, append to it:
                        existing_chat.append(chat);
                    } else {
                        // If chat does not exist, add a new one:
                        chats.insert(u.id, chat);
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}