use image::ImageFormat;
use jemini::{Chat, ImageData, JeminiClient as GeminiClient};
use log::{debug, error};
use poise::serenity_prelude::{self as serenity, UserId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

///Toggles the bot's listening/not
static ACTIVE: AtomicBool = AtomicBool::new(false);

///Always good to know your bot's ID.
const BOT_ID: u64 = 1209058891556724746;

struct Data {
    gemini_client: GeminiClient,

    #[allow(dead_code)]
    chats: Arc<Mutex<HashMap<UserId, Chat>>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'c> = poise::Context<'c, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn activate_bot(ctx: Context<'_>) -> Result<(), Error> {
    ACTIVE.store(true, Ordering::SeqCst);
    _ = ctx.reply("GeminiBot will see you now...").await;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn deactivate_bot(ctx: Context<'_>) -> Result<(), Error> {
    _ = ctx.reply("ciao!").await;
    ACTIVE.store(false, Ordering::SeqCst);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();

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
            commands: vec![activate_bot(), deactivate_bot()],
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
    if !ACTIVE.load(Ordering::SeqCst) {
        return Ok(());
    }

    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            debug!("Logged in as {:#?}", data_about_bot.user);
        }
        serenity::FullEvent::Message { new_message } => {
            //TODO: handle longer lived chats and their history.
            // Check if chat exists for the user:
            // let u = &new_message.author;
            // let chats = data.chats.lock().await;
            // if let Some(existing_chat) = chats.get_mut(&u.id) {
            //     // If chat exists, append to it:
            //     existing_chat.append(chat);
            // } else {
            //     // If chat does not exist, add a new one:
            //     // chats.insert(u.id, chat);
            // }

            if new_message.author.id != UserId::new(BOT_ID) {
                let gemini_client = &data.gemini_client;

                // Handle img:
                if let Some(attachment) = new_message.attachments.first() {
                    if let Ok(content) = attachment.download().await {
                        debug!("filename:{}", attachment.filename);
                        debug!("img_size: {}b", content.len());
                        let format = match PathBuf::from(&attachment.filename).extension() {
                            Some(ext) => match ext.to_str() {
                                Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
                                Some("png") => ImageFormat::Png,
                                Some("gif") => ImageFormat::Gif,
                                Some("webp") => ImageFormat::WebP,
                                _ => {
                                    error!("Unsupported image format");
                                    return Ok(());
                                }
                            },
                            None => {
                                error!("No file extension found");
                                return Ok(());
                            }
                        };
                        debug!("{}", format.to_mime_type());
                        if let Ok(image_data) = ImageData::from_bytes(content.into(), format) {
                            if let Ok(response) = gemini_client
                                .text_and_image(&new_message.content, image_data)
                                .await
                            {
                                let timer = std::time::Instant::now();
                                debug!(
                                    "{:#?}\n... in {:?}s",
                                    response.most_recent(),
                                    timer.elapsed().as_secs()
                                );

                                _ = new_message
                                    .reply(
                                        ctx,
                                        response.most_recent().unwrap_or({
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

                // Handle @mentions
                if new_message
                    .mentions
                    .iter()
                    .any(|u| u.id == UserId::new(BOT_ID))
                {
                    let timer = std::time::Instant::now();
                    // Remove the @mention
                    let (_, mention_stripped) = &new_message
                        .content
                        .split_once('@')
                        .to_owned()
                        .unwrap_or_default();
                    let chat = gemini_client.new_chat(mention_stripped).await?;
                    debug!(
                        "{:#?}\n... in {:?}s",
                        chat.most_recent(),
                        timer.elapsed().as_secs()
                    );

                    new_message.reply(ctx, chat.most_recent().unwrap()).await?;
                }
            }
        }
        _ => {}
    }

    Ok(())
}
