extern crate dotenv;

use dotenv::dotenv;
use std::env;

use binance::account::*;
use binance::api::*;
use binance::model::*;

use std::fmt::{Display, Error, Formatter};

pub struct Credentials {
    pub api_key: String,
    pub secret_key: String,
}

struct BalVec(Vec<Balance>);

impl Display for BalVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut line_separated = String::new();

        for bal in &self.0 {
            let sum: f32 = &bal.free.parse::<f32>().unwrap() + &bal.locked.parse::<f32>().unwrap();

            line_separated.push_str(&format!("{:>7} |", &bal.asset));
            line_separated.push_str(&format!("{:>12} free", &bal.free));
            line_separated.push_str(&format!("{:>12} locked", &bal.locked));
            line_separated.push_str(&format!(" | = {} \n", sum));
        }
        write!(f, "{}", line_separated)
    }
}

fn main() {
    dotenv().ok();

    let creds = Credentials {
        api_key: env::var("BINANCE_API_KEY").expect("Failed to read BINANCE_API_KEY"),
        secret_key: env::var("BINANCE_SECRET_KEY").expect("Failed to read BINANCE_SECRET_KEY"),
    };

    let account: Account = Binance::new(Some(creds.api_key), Some(creds.secret_key));

    match account.get_account() {
        Ok(answer) => {
            let balances = BalVec(
                answer
                    .balances
                    .into_iter()
                    .filter(|bal| bal.free.parse::<f32>().unwrap() > 0.)
                    .collect(),
            );
            println!("{}", balances)
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
