use mongodb::{bson::{DateTime, Uuid}, Collection};
use poise::{serenity_prelude::{CreateEmbed, CreateMessage, PermissionOverwrite, PermissionOverwriteType, Permissions}, ApplicationContext, CreateReply, Modal};

use crate::{models::ticket::Ticket, utils::{channel::create_text_channel, color::GREEN, EVERYONE_ID, SUPPORT_ID}, Context, Data, Error};

#[derive(Debug, poise::Modal)]
#[allow(dead_code)]
#[name = "Ticket Creation"]
struct TicketCreation {
    #[name = "Reason for Ticket"]
    #[placeholder = "Please enter a reason for your ticket!"]
    reason_input: String,
}

#[poise::command(slash_command)]
pub async fn openticket(ctx: ApplicationContext<'_, Data, Error>) -> Result<(), Error> {
    // open the modal for the ticket creation system
    let model_data = TicketCreation::execute(ctx).await?.unwrap();

    // get the db and collection for tickets
    let db = &ctx.data().db;
    let tickets_col: Collection<Ticket> = db.database("ticket").collection("tickets");

    // ids and names for ticket creation
    let uuid = Uuid::new();
    let ticket_id = uuid.to_string();
    let short_id = &ticket_id[..6]; // use first 6 chars of the uuid

    let guild = ctx.guild_id().unwrap();
    let channel_name = format!("ticket-{}-{}", ctx.author().name, short_id);

    let user_id = ctx.author().id;

    // permissions for the class
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(user_id),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(SUPPORT_ID),
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(EVERYONE_ID),
        }
    ];

    // create the channel
    let serenity_ctx = ctx.serenity_context();
    let new_channel_id = create_text_channel(guild, channel_name, permissions, serenity_ctx).await?;
    
    // ticket model
    let ticket = Ticket {
        user_id: ctx.author().id.to_string(),
        username: ctx.author().name.clone(),
        ticket_id: ticket_id.clone(),
        created_at: DateTime::now(),
        status: true,
        reason: model_data.reason_input.clone(),
        messages: vec![],
        channel_id: new_channel_id.to_string(),
    };

    Ticket::create_ticket(ticket, &tickets_col).await?;

    // send start message in the channel
    let start_embed = CreateEmbed::default()
    .title(format!("Ticket ID: {}", ticket_id))
    .description(format!(
        "<@{}> This is your ticket, below are the stats you provided! If you have any other information please provide it while waiting for a support team member.",
        user_id.to_string()
    ))
    .field("Ticket Reason:", model_data.reason_input.clone(), true)
    .color(GREEN);

    new_channel_id.send_message(ctx.http(), 
        CreateMessage::default()
            .content(format!("Hey, <@{}>", user_id))
            .embed(start_embed)
    ).await?;

    // embed response
    let embed = CreateEmbed::default()
    .description(format!("Ticket has been created! Channel: <#{}>", new_channel_id.to_string()))
    .field("Reason", model_data.reason_input.clone(), false)
    .color(GREEN);
    ctx.send(CreateReply::default()
        .embed(embed)
        .ephemeral(true)).await?;


    Ok(()) // :thumbsup:
}