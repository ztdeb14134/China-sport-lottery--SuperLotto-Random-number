#[allow(non_snake_case)]
mod China_sports_lottery;
use std::env;

use crate::China_sports_lottery::PrintResult;
use crate::China_sports_lottery::*;
use chrono::prelude::*;
fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 1
        && (args[1] == "test" || args[1] == "-test" || args[1] == "--test" || args[1] == "-t")
    {
        SuperLotto::fast_test();
        return;
    }
    //26 + 18 + 18 + 18 + 40 = 118¥
    match Local::now().date_naive().weekday() {
        chrono::Weekday::Mon => {
            // 26¥
            let mut super_lotto = SuperLotto::new(PlayType::KeyFiller(5, 0, 1, 11), 1, false);
            super_lotto.draw().printout();
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
        }
        chrono::Weekday::Tue => {
            // 18¥
            SuperLotto::all_cast_seven().printout();
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
        }
        chrono::Weekday::Wed => {
            // 18¥
            let mut super_lotto = SuperLotto::new(PlayType::Single, 2, true);
            super_lotto.set_multiple(3);
            super_lotto.draw().printout();
        }
        chrono::Weekday::Thu => {
            // 18¥
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
            let mut super_lotto = SuperLotto::new(PlayType::Duplex(6, 2), 1, false);
            super_lotto.draw().printout();
        }
        chrono::Weekday::Fri => {
            // 40¥
            let mut super_lotto = SuperLotto::new(PlayType::KeyFiller(2, 5, 1, 2), 1, false);
            super_lotto.draw().printout();
        }
        _ => println!("节制"),
    }
    println!("Enter to quit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test0() {
//         let mut sl = SuperLotto::new(PlayType::Single, 1, false);
//         sl.draw().printout();
//     }
// }
