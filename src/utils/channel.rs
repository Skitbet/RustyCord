
use poise::serenity_prelude::{self, ChannelId, CreateChannel, GuildId, PermissionOverwrite};

use crate::Error;

pub async fn create_text_channel(
    guild: GuildId,
    name: String,
    permissions: Vec<PermissionOverwrite>,
    ctx: &serenity_prelude::Context,
) -> Result<ChannelId, Error> {
    let new_channel = guild
    .create_channel(&ctx.http, 
        CreateChannel::new(name)
        .permissions(permissions)
        .kind(poise::serenity_prelude::ChannelType::Text)
    )
    .await?;

    Ok(new_channel.id)
}