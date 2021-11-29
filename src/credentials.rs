extern crate dotenv;

pub struct Credentials {
    pub api_key: String,
    pub secret_key: String,
}

pub fn read_credentials() -> Credentials {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    Credentials {
        api_key: env::var("BINANCE_API_KEY").expect("Failed to read BINANCE_API_KEY"),
        secret_key: env::var("BINANCE_SECRET_KEY").expect("Failed to read BINANCE_SECRET_KEY"),
    }
}
