#[allow(non_snake_case)]
mod China_Welfare_Lottery;
#[allow(non_snake_case)]
mod China_sports_lottery;
use crate::{
    China_Welfare_Lottery::{HappyEight, RBball, SevenLotto},
    China_sports_lottery::*,
};
use chrono::prelude::*;
fn main() {
    match Local::now().date_naive().weekday() {
        chrono::Weekday::Mon => {
            let super_lotto = SuperLotto::new(5, 4, 1);
            super_lotto.run().unwrap();
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
            println!("+18大乐透套餐");
        }
        chrono::Weekday::Tue => {
            let super_lotto = SuperLotto::new(5, 3, 2);
            super_lotto.run().unwrap();
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
        }
        chrono::Weekday::Wed => {
            let super_lotto = SuperLotto::new(5, 2, 2);
            super_lotto.run().unwrap();
            println!("全部追加")
        }
        chrono::Weekday::Thu => {
            println!("12 13 19 25 34 : 05 06\n07 09 13 19 27 : 04 08");
            let super_lotto = SuperLotto::new(6, 2, 1);
            super_lotto.run().unwrap();
        }
        chrono::Weekday::Fri => {
            println!("胆码");
            let super_lotto = SuperLotto::new(2, 1, 1);
            super_lotto.run().unwrap();
            println!("托码");
            let super_lotto = SuperLotto::new(5, 2, 1);
            println!("托码单独打一注追加");
            super_lotto.run().unwrap();
        }
        chrono::Weekday::Sat => println!("节制"),
        chrono::Weekday::Sun => println!("节制"),
    }

    if false {
        let happy_eight = HappyEight::new(10, 1);
        let rbball = RBball::new(6, 1, 1);
        let sevenlotto = SevenLotto::new(7, 1);
        happy_eight.run().unwrap();
        rbball.run().unwrap();
        sevenlotto.run().unwrap();
    }
}
