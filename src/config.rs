use std::{env, error::Error};

use dotenv::dotenv;

pub struct Config {
    pub token: String,
    pub mongo_uri: String,
}

impl Config {
    pub fn from_dotenv() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let token = env::var("TOKEN").expect("Expected a TOKEN value in the enviroment variables!");
        let mongo_uri = env::var("MONGO_URI").expect("Expected a MONGO_URI value in the enviroment variables!");

        Ok(Self {
            token,
            mongo_uri,
        })
    }
}