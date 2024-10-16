use std::vec;

use crate::{Data, Error};

pub mod age;
pub mod tickets;

pub fn get_commands() -> Vec<poise::Command<Data, Error>>{
    return vec![
        age::age(),
        tickets::open_ticket(),
    ]
}