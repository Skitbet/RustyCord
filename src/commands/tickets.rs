use mongodb::{bson::{DateTime, Uuid}, Collection};
use poise::{serenity_prelude::{CreateEmbed, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId}, CreateReply};

use crate::{models::ticket::Ticket, utils::channel::create_text_channel, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn open_ticket(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().db;
    let tickets_col: Collection<Ticket> = db.database("ticket").collection("tickets");

    let uuid = Uuid::new();
    let ticket_id = uuid.to_string();
    let short_id = &ticket_id[..6]; // use first 6 chars of the uuid

    let guild = ctx.guild_id().unwrap();
    let channel_name = format!("ticket-{}-{}", ctx.author().name, short_id);

    let everyone_role = RoleId::new(1119133268235788328); // there gotta be a better way to get default role?
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

    let new_channel_id = create_text_channel(guild, channel_name, permissions, &ctx).await?;

    let ticket = Ticket {
        user_id: ctx.author().id.to_string(),
        username: ctx.author().name.clone(),
        ticket_id: ticket_id.clone(),
        created_at: DateTime::now(),
        status: "Open".to_string(),
        messages: vec![],
        channel_id: new_channel_id.to_string(),
    };

    Ticket::create_ticket(ticket, &tickets_col).await?;

    let embed = CreateEmbed::default()
    .description(format!("Ticket opened successfully, channel created: <#{}>", new_channel_id.to_string()));

    ctx.send(CreateReply::default()
        .embed(embed)).await?;

    Ok(())
}