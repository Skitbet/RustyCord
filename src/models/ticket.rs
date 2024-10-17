use mongodb::{bson::{doc, DateTime}, Collection};
use poise::serenity_prelude::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub user_id: String,
    pub username: String,
    pub ticket_id: String,
    pub created_at: DateTime,
    pub status: bool,
    pub reason: String,
    pub messages: Vec<String>,
    pub channel_id: String,
}

impl Ticket {
    pub async fn create_ticket(ticket: Ticket, tickets_col: &Collection<Ticket>) -> Result<(), mongodb::error::Error> {
        tickets_col.insert_one(ticket).await?;
        Ok(())
    }

    pub async fn close_ticket_by_chan(channel: ChannelId, ticket_col: &Collection<Ticket>) -> Result<bool, mongodb::error::Error> {
        let filter = doc! { "channel_id": channel.to_string() as String };

        if let Some(_ticket) = ticket_col.find_one(filter.clone()).await? {
            // ticket found so delete it
            let result = ticket_col.delete_one(filter).await?;
            
            if result. deleted_count > 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }
}