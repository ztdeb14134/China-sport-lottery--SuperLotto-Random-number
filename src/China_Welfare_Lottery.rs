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
pub struct HappyEight {
    /*
     * choose Ten
     * 0   75,394,027,566/1,646,492,110,120
     * 5   84,675,282,048/1,646,492,110,120
     * 6   18,900,732,600/1,646,492,110,120
     * 7   2,652,734,400 /1,646,492,110,120
     * 8   222,966,900   /1,646,492,110,120
     * 9   10,077,600    /1,646,492,110,120
     * 10  184,756       /1,646,492,110,120
     */
    duplex: usize,
    number: usize,
}
impl HappyEight {
    pub fn new(_duplex: usize, _number: usize) -> Self {
        HappyEight {
            duplex: _duplex,
            number: _number,
        }
    }
    pub fn run(&self) -> Result<(), ()> {
        let mut result: Vec<Vec<String>> = Vec::new();
        for x in 0..self.number {
            let progress = Arc::new(AtomicUsize::new(0));
            let progress_clone = Arc::clone(&progress);
            println!("开始第{}次roll号码", x + 1);
            let mut first: Vec<String> = (1..=80).map(|num| format!("{:02}", num)).collect();
            let mut rng: rand::prelude::ThreadRng = rand::rng();

            let times: usize = rng.random_range(88888888..888888888);

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
                        .truecolor(
                            (200.0 * (1.0 - percent)) as u8,
                            (255.0 * percent) as u8,
                            100,
                        )
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
                progress.fetch_add(1, Ordering::Relaxed);
            }
            result.push(first.iter().take(self.duplex).cloned().collect());
            handle.join().unwrap();
        }
        println!("{:?}", result);
        println!("按Enter退出");
        io::stdin().read_line(&mut String::new()).unwrap();
        Ok(())
    }
}
pub struct RBball {
    /*
     * 6+1 1/17,721,088
     * 6+0 15/17,721,088
     * 5+1 162/17,721,088
     * 5+0 4+1 7,695/17,721,088
     */
    pub front: usize,
    pub behind: usize,
    pub number: usize,
}
impl RBball {
    pub fn new(_front: usize, _behind: usize, _number: usize) -> Self {
        RBball {
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
            println!("开始第{}次roll号码", x + 1);
            let mut first: Vec<String> = (1..=33).map(|num| format!("{:02}", num)).collect();
            let mut last: Vec<String> = (1..=16).map(|num| format!("{:02}", num)).collect();
            let mut rng: rand::prelude::ThreadRng = rand::rng();

            let times: usize = rng.random_range(88888888..888888888);

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
                        .truecolor(
                            (200.0 * (1.0 - percent)) as u8,
                            (255.0 * percent) as u8,
                            100,
                        )
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
            this_result.push(":".to_string());
            this_result.append(&mut last.iter().take(self.behind).cloned().collect());
            result.push(this_result);
            handle.join().unwrap();
        }
        println!("{:?}", result);
        println!("按Enter退出");
        io::stdin().read_line(&mut String::new()).unwrap();
        Ok(())
    }
}

pub struct SevenLotto {
    /*
     *  7+0  1/2,035,800
     *  6+1  7/2,035,800
     *  6+0  154/2,035,800
     *  5+1  462/2,035,800
     *  5+0  4,851/2,035,800
     *  4+1  8,085/2,035,800
     *  4+0  53,900/2,035,800
     */
    duplex: usize,
    number: usize,
}
impl SevenLotto {
    pub fn new(_duplex: usize, _number: usize) -> Self {
        SevenLotto {
            duplex: _duplex,
            number: _number,
        }
    }
    pub fn run(&self) -> Result<(), ()> {
        let mut result: Vec<Vec<String>> = Vec::new();
        for x in 0..self.number {
            let progress = Arc::new(AtomicUsize::new(0));
            let progress_clone = Arc::clone(&progress);
            println!("开始第{}次roll号码", x + 1);
            let mut first: Vec<String> = (1..=30).map(|num| format!("{:02}", num)).collect();
            let mut rng: rand::prelude::ThreadRng = rand::rng();

            let times: usize = rng.random_range(88888888..888888888);

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
                        .truecolor(
                            (200.0 * (1.0 - percent)) as u8,
                            (255.0 * percent) as u8,
                            100,
                        )
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
                progress.fetch_add(1, Ordering::Relaxed);
            }
            result.push(first.iter().take(self.duplex).cloned().collect());
            handle.join().unwrap();
        }
        println!("{:?}", result);
        println!("按Enter退出");
        io::stdin().read_line(&mut String::new()).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod lottery_test {
    #[test]
    fn test1() {}
}
