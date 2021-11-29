pub mod binance {
    use crate::credentials::*;
    use binance::account::*;
    use binance::api::*;

    pub fn get_api() -> Account {
        let creds: Credentials = read_credentials();
        Binance::new(Some(creds.api_key), Some(creds.secret_key))
    }
}
