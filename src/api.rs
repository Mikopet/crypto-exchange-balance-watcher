pub mod binance {
    use binance::account::*;
    use binance::api::*;
    use crate::credentials::*;

    pub fn get_api() -> Account {
        let creds: Credentials = read_credentials();
        Binance::new(Some(creds.api_key), Some(creds.secret_key))
    }
}
