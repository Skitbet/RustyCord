use mongodb::{bson::DateTime, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub user_id: String,
    pub username: String,
    pub ticket_id: String,
    pub created_at: DateTime,
    pub status: String,
    pub messages: Vec<String>,
    pub channel_id: String,
}

impl Ticket {
    pub async fn create_ticket(ticket: Ticket, tickets_col: &Collection<Ticket>) -> Result<(), mongodb::error::Error> {
        tickets_col.insert_one(ticket).await?;
        Ok(())
    }
}