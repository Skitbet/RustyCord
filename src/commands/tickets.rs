use std::fmt::format;

use mongodb::{bson::{uuid, DateTime, Uuid}, Collection};
use poise::{serenity_prelude::{self, CreateChannel, CreateEmbed, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId}, CreateReply};

use crate::{models::ticket::Ticket, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn open_ticket(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().db;
    let tickets_col: Collection<Ticket> = db.database("ticket").collection("tickets");

    let uuid = Uuid::new();
    let ticket_id = uuid.to_string();
    let short_id = &ticket_id[..6]; // use first 6 chars of the uuid

    let guild = ctx.guild_id().unwrap();
    let channel_name = format!("ticker-{}", short_id);

    let everyone_role = RoleId::new(1119133268235788328);
    let staff_role_id = RoleId::new(1119135390574575686);
    let user_id = ctx.author().id;


    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(user_id),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(staff_role_id),
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(everyone_role),
        }
    ];

    let new_channel = guild
        .create_channel(&ctx.serenity_context().http,
            CreateChannel::new(channel_name)
            .permissions(permissions)
            .kind(serenity_prelude::ChannelType::Text)
    ).await?;

    let ticket = Ticket {
        user_id: ctx.author().name.clone(),
        username: ctx.author().name.clone(),
        ticket_id: ticket_id.clone(),
        created_at: DateTime::now(),
        status: "Open".to_string(),
        messages: vec![],
        channel_id: new_channel.id.to_string(),
    };

    let embed = CreateEmbed::default()
    .description(format!("Ticket opened successfully, channel created: <#{}>", new_channel.id));

    tickets_col.insert_one(ticket).await?;
    ctx.send(CreateReply::default()
        .embed(embed)).await?;

    Ok(())
}