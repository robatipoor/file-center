pub mod constants;

use dotenv::dotenv;
use envy::from_env;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub database_mode: String,
    pub secret_key: String,
    pub rust_log: String,
    pub path_file: String,
    pub address_server: String,
}

fn get_config() -> Config {
    dotenv().ok();
    match from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
