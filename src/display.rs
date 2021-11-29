use binance::model::*;
use std::fmt::{Display, Error, Formatter};

pub struct BalVec(Vec<Balance>);

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

impl BalVec {
    pub fn new(balances: Vec<Balance>) -> BalVec {
        BalVec(balances)
    }
}
