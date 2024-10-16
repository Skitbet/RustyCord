use poise::serenity_prelude::{self as serenity, User};

mod config;

struct Data{} // bot data like users
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<User>
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.reply(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let config = config::Config::from_dotenv().expect("Failed to load config.");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        commands: vec![age()],
        ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {})
        })
    })
    .build();

    let client = serenity::ClientBuilder::new(config.token, intents)
    .framework(framework)
    .await;
    client.unwrap().start().await.unwrap();
}
