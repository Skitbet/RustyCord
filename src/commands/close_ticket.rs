use mongodb::Collection;
use poise::{ApplicationContext, CreateReply};

use crate::{models::ticket::Ticket, Data, Error};

#[poise::command(slash_command)]
pub async fn closeticket(ctx: ApplicationContext<'_, Data, Error>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let channel = ctx.channel_id();

    // get the db and collection for tickets
    let db = &ctx.data().db;
    let tickets_col: Collection<Ticket> = db.database("ticket").collection("tickets");

    let result = Ticket::close_ticket_by_chan(channel, &tickets_col).await;
    match result {
        Ok(true) => {
            channel.delete(ctx.http()).await?;
        }
        Ok(false) => {
            ctx.send(CreateReply::default().content("This channel is not a ticket!")).await?;
        }
        Err(err) => eprintln!("An error occurred: {}", err),
    }

    Ok(())
}