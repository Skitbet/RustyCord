use std::vec;

use crate::{Data, Error};

pub mod age;

pub fn get_commands() -> Vec<poise::Command<Data, Error>>{
    return vec![
        age::age()
    ]
}