use poise::{serenity_prelude::{CreateEmbed, Embed, User}, CreateReply};

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<User>
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let embed = CreateEmbed::default()
    .title(format!("User info for {}", u.name))
    .description(format!("Here is the User Info for the user {}", u.name))
    .thumbnail(u.avatar_url().unwrap())
    .field("Created", u.created_at().to_string(), false)
    .color(0x00ff00)
    ;

    let reply = poise::CreateReply::default()
    .embed(embed);

    ctx.send(reply).await?;
    Ok(())
}