use colored::Colorize;
use rand::{self, Rng, seq::SliceRandom};
use std::{
    io::{self, Write, stdout},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};
pub struct SuperLotto {
    /*
     *5+2                   1/21,425,712
     *5+1                   20/21,425,712
     *5+0                   45/21,425,712
     *4+2                   150/21,425,712
     *4+1                   3000/21,425,712
     *3+2                   4350/21,425,712
     *4+0                   6750/21,425,712
     *3+1 2+2               127,600/21,425,712
     *3+0 2+1 1+2 0+2       1,777,661/21,425,712
     */
    pub front: usize,
    pub behind: usize,
    pub number: usize,
}
impl SuperLotto {
    pub fn new(_front: usize, _behind: usize, _number: usize) -> Self {
        SuperLotto {
            front: _front,
            behind: _behind,
            number: _number,
        }
    }

    pub fn run(&self) -> Result<(), ()> {
        let mut result: Vec<Vec<String>> = Vec::new();
        for x in 0..self.number {
            let progress = Arc::new(AtomicUsize::new(0));
            let progress_clone = Arc::clone(&progress);
            println!("\n开始第{}次roll号码", x + 1);
            let mut first: Vec<String> = (1..=35).map(|num| format!("{:02}", num)).collect();
            let mut last: Vec<String> = (1..=12).map(|num| format!("{:02}", num)).collect();
            let mut rng: rand::prelude::ThreadRng = rand::rng();

            let times: usize = rng.random_range(19770801..20130330);

            let handle = thread::spawn(move || {
                loop {
                    let val = progress_clone.load(Ordering::Relaxed);
                    if val == times {
                        break;
                    }
                    let percent: f64 = val as f64 / times as f64;

                    let filled = (percent * 50.0).round() as usize;
                    let empty = 50 - filled;
                    let bar: String = "\u{25a0}"
                        .repeat(filled)
                        .truecolor(100, (255.0 * percent) as u8, 100)
                        .to_string();
                    let space = " ".repeat(empty);
                    print!(
                        "\r{}{} {:>3}",
                        bar,
                        space,
                        (percent * 100.0).round() as usize
                    );
                    stdout().flush().unwrap();
                }
            });
            for _ in 0..times {
                first.shuffle(&mut rng);
                last.shuffle(&mut rng);
                progress.fetch_add(1, Ordering::Relaxed);
            }
            let mut this_result: Vec<String> = first.iter().take(self.front).cloned().collect();
            let mut c: Vec<String> = last.iter().take(self.behind).cloned().collect();

            this_result.sort();
            c.sort();

            this_result.push(":".to_string());
            this_result.append(&mut c);
            result.push(this_result);
            handle.join().unwrap();
        }
        println!();
        result.iter().for_each(|x| println!("{:?}", x));
        println!("按Enter退出");
        io::stdin().read_line(&mut String::new()).unwrap();
        Ok(())
    }
}
