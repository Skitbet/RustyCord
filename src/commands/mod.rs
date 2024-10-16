use std::vec;

use crate::{Data, Error};

pub mod age;
pub mod create_ticket;
pub mod close_ticket;

pub fn get_commands() -> Vec<poise::Command<Data, Error>>{
    return vec![
        age::age(),
        create_ticket::openticket(),
        close_ticket::closeticket(),
    ]
}