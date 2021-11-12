mod api;
mod credentials;
mod display;

use display::BalVec;

fn main() {
    match api::binance::get_api().get_account() {
        Ok(answer) => {
            let balances = BalVec::new(
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
