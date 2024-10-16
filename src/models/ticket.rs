use mongodb::bson::DateTime;
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