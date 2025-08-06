#[allow(non_snake_case)]
mod China_sports_lottery;
mod args;
mod printtrait;
use std::env;

use crate::China_sports_lottery::*;
use crate::args::args_handle;
use crate::printtrait::PrintResult;
use chrono::prelude::*;

macro_rules! protect_code {
    () => {
        [
            "12 13 19 25 34 : 05 06",
            "07 09 13 19 27 : 04 08",
            "03 09 20 21 28 : 03 04", //420683_20040921_0328
            "03 14 20 23 30 : 03 04", //410303_20040804_5236
        ]
        .printout();
    };
}
fn main() {
    if args_handle(env::args().collect::<Vec<_>>()) {
        return;
    }
    match Local::now().date_naive().weekday() {
        chrono::Weekday::Mon => {
            let mut super_lotto = SuperLotto::new(PlayType::KeyFiller(0, 5, 1, 11), 1, false);
            super_lotto.draw().printout();
            protect_code!();
        }
        chrono::Weekday::Tue => {
            SuperLotto::all_cast_seven().printout();
        }
        chrono::Weekday::Wed => {
            let mut super_lotto = SuperLotto::new(PlayType::Single, 2, true);
            super_lotto.set_multiple(3);
            super_lotto.draw().printout();
            protect_code!();
        }
        chrono::Weekday::Thu => {
            let mut super_lotto = SuperLotto::new(PlayType::Duplex(6, 2), 1, false);
            super_lotto.draw().printout();
        }
        chrono::Weekday::Fri => {
            let mut super_lotto = SuperLotto::new(PlayType::KeyFiller(2, 5, 1, 2), 1, false);
            super_lotto.draw().printout();
            protect_code!();
        }
        _ => println!("节制"),
    }
    println!("Enter to quit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
