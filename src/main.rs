extern crate dotenv;

use dotenv::dotenv;
use std::env;

use binance::account::*;
use binance::api::*;

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

    let account: Account = Binance::new(Some(creds.api_key), Some(creds.secret_key));

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }

}
