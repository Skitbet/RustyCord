use std::sync::Arc;

use mongodb::Client;
use poise::serenity_prelude::{self as serenity, ActivityData};


mod config;
mod commands;
mod models;
mod utils;

struct Data{
    db: Arc<Client>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let config = config::Config::from_dotenv().expect("Failed to load config.");
    let intents = serenity::GatewayIntents::non_privileged();
    
    let mongo_client = Client::with_uri_str(&config.mongo_uri).await.expect("Failed to initalize MongoDB client.");
    let db = Arc::new(mongo_client);

    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        commands: commands::get_commands(),
        ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {
                db: db.clone(),
            })
        })
    })
    .build();

    let client = serenity::ClientBuilder::new(config.token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            ctx.set_activity(Some(ActivityData::watching("Stimky Wumbo")));
        }
        _ => {}
    }
    Ok(())
}
