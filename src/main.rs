#[allow(non_snake_case)]
mod China_sports_lottery;
use crate::China_sports_lottery::*;
use chrono::prelude::*;
fn main() {
    match Local::now().date_naive().weekday() {
        chrono::Weekday::Mon => {
            let mut super_lotto = SuperLotto::new(PlayType::KeyFiller(5, 0, 1, 11), 1, false);
            let _ = super_lotto
                .draw()
                .unwrap()
                .into_iter()
                .map(|s| println!("{}", s));
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
        }
        chrono::Weekday::Tue => {
            let mut super_lotto = SuperLotto::new(PlayType::Duplex(5, 3), 2, false);
            let _ = super_lotto
                .draw()
                .unwrap()
                .into_iter()
                .map(|s| println!("{}", s));
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
        }
        chrono::Weekday::Wed => {
            let mut super_lotto = SuperLotto::new(PlayType::Single, 2, true);
            super_lotto.set_multiple(3);
            let _ = super_lotto
                .draw()
                .unwrap()
                .into_iter()
                .map(|s| println!("{}", s));
        }
        chrono::Weekday::Thu => {
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
            let mut super_lotto = SuperLotto::new(PlayType::Duplex(6, 2), 1, false);
            let _ = super_lotto
                .draw()
                .unwrap()
                .into_iter()
                .map(|s| println!("{}", s));
        }
        chrono::Weekday::Fri => {
            let mut super_lotto = SuperLotto::new(PlayType::KeyFiller(2, 5, 1, 2), 1, false);
            let _ = super_lotto
                .draw()
                .unwrap()
                .into_iter()
                .map(|s| println!("{}", s));
        }
        _ => println!("节制"),
    }
}
