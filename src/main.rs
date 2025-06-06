use colored::Colorize;
use rand::{self, Rng, seq::SliceRandom};
use std::{
    io::{self, Write, stdout},
    thread::sleep,
    time::Duration,
};
fn main() {
    for x in 0..10 {
        println!("开始第{}次roll号码", x);
        let mut rng = rand::rng();
        let mut first: Vec<i32> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
        ];
        let mut last: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let times = rng.random_range(100..10000);
        for i in 0..times {
            let percent = i as f64 / times as f64;
            let filled = (percent * 50.0).round() as usize;
            let empty = 50 - filled;
            let bar: String = "\u{25a0}"
                .repeat(filled)
                .truecolor(
                    (255.0 * percent) as u8,
                    (200.0 * (1.0 - percent)) as u8,
                    100,
                )
                .to_string();
            let space = " ".repeat(empty);
            print!("\r{}{} {:>3}", bar, space, (percent * 100.0).round() as u32);
            stdout().flush().unwrap();
            sleep(Duration::from_millis(20));
            first.shuffle(&mut rng);
            last.shuffle(&mut rng);
        }

        let mut first: Vec<i32> = first.iter().take(5).cloned().collect();
        let mut last: Vec<i32> = last.iter().take(2).cloned().collect();
        first.sort();
        last.sort();
        println!("第{}次roll号结果为:", x);
        first.iter().for_each(|x| print!("{} ", x));
        last.iter().for_each(|x| print!("{} ", x));
        println!();
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}
