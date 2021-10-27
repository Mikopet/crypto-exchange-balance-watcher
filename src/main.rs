extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Credentials {
    pub api_key: String,
    pub secret_key: String,
}

fn main() {
    dotenv().ok();

    let creds = Credentials {
        api_key: env::var("BINANCE_API_KEY").expect("Failed to read BINANCE_API_KEY"),
        secret_key: env::var("BINANCE_SECRET_KEY").expect("Failed to read BINANCE_SECRET_KEY"),
    };

    println!("{:?}", creds);
}
