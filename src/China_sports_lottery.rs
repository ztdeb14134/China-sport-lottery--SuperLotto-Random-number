use colored::Colorize;
use rand::{self, Rng, seq::SliceRandom};
use std::{
    io::{Write, stdout},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};
#[derive(Debug)]
pub enum PlayType {
    Single,
    Duplex(usize, usize),
    KeyFiller(usize, usize, usize, usize),
}
#[derive(Debug)]
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
    pub multiple: Option<usize>,
    pub play_type: PlayType,
    pub number: usize,
    pub Additional: bool,
}
pub trait PrintResult {
    fn printout(self);
}
impl PrintResult for Result<Vec<String>, ()> {
    fn printout(self) {
        self.unwrap().into_iter().for_each(|s| println!("{}", s));
    }
}
impl SuperLotto {
    pub fn new(play_type: PlayType, number: usize, Additional: bool) -> Self {
        SuperLotto {
            multiple: None,
            play_type,
            number,
            Additional,
        }
    }
    pub fn set_multiple(&mut self, multiple: usize) {
        self.multiple = Some(multiple);
    }
    pub fn draw(&mut self) -> Result<Vec<String>, ()> {
        let mut result: Vec<String> = Vec::new();
        for x in 0..self.number {
            let progress = Arc::new(AtomicUsize::new(0));
            let progress_clone = Arc::clone(&progress);
            let mut first: Vec<String> = (1..=35).map(|num| format!("{:02}", num)).collect();
            let mut last: Vec<String> = (1..=12).map(|num| format!("{:02}", num)).collect();
            let mut rng: rand::prelude::ThreadRng = rand::rng();
            let times: usize = rng.random_range(19770801..20130330);
            let handle = Some(thread::spawn(move || {
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
                        "\r第{}次号码：{}{} {:>3}",
                        x + 1,
                        bar,
                        space,
                        (percent * 100.0).round() as usize
                    );
                    stdout().flush().unwrap();
                }
            }));
            match self.play_type {
                PlayType::Single => {
                    for _ in 0..times {
                        first.shuffle(&mut rng);
                        last.shuffle(&mut rng);
                        progress.fetch_add(1, Ordering::Relaxed);
                    }
                    let mut front_result: Vec<String> = first.iter().take(5).cloned().collect();
                    let mut behind_result: Vec<String> = last.iter().take(2).cloned().collect();
                    front_result.sort();
                    behind_result.sort();
                    result.push(format!(
                        "第{}注: {} : {}{}",
                        x + 1,
                        front_result.join(" "),
                        behind_result.join(" "),
                        format!(
                            "{}",
                            match self.multiple {
                                Some(m) => format!(
                                    "  {}倍{}",
                                    m,
                                    if self.Additional { "追加" } else { "" }
                                ),
                                None => format!("{}", if self.Additional { "  追加" } else { "" }),
                            }
                        )
                    ));
                }
                PlayType::Duplex(front, behind) => {
                    for _ in 0..times {
                        first.shuffle(&mut rng);
                        last.shuffle(&mut rng);
                        progress.fetch_add(1, Ordering::Relaxed);
                    }
                    let mut front_result: Vec<String> = first.iter().take(front).cloned().collect();
                    let mut behind_result: Vec<String> =
                        last.iter().take(behind).cloned().collect();
                    front_result.sort();
                    behind_result.sort();
                    result.push(format!(
                        "第{}注: {} : {}{}",
                        x + 1,
                        front_result.join(" "),
                        behind_result.join(" "),
                        format!(
                            "{}",
                            match self.multiple {
                                Some(m) => format!(
                                    "  {}倍{}",
                                    m,
                                    if self.Additional { "追加" } else { "" }
                                ),
                                None => format!("{}", if self.Additional { "  追加" } else { "" }),
                            }
                        )
                    ));
                }
                PlayType::KeyFiller(key_front, filler_front, key_behind, filler_behind) => {
                    for _ in 0..times {
                        first.shuffle(&mut rng);
                        last.shuffle(&mut rng);
                        progress.fetch_add(1, Ordering::Relaxed);
                    }
                    let mut keyfront: Vec<String> = first.drain(0..key_front).collect();
                    let mut keybehind_result: Vec<String> = last.drain(0..key_behind).collect();
                    keyfront.sort();
                    keybehind_result.sort();
                    let mut filler_front_result: Vec<String> =
                        first.drain(0..filler_front).collect();
                    let mut filler_behind_result: Vec<String> =
                        last.drain(0..filler_behind).collect();
                    filler_front_result.sort();
                    filler_behind_result.sort();

                    result.push(format!(
                        "第{}注{}:\n前区胆:{}\n前区拖:{}\n后区胆:{}\n后区拖:{}",
                        x + 1,
                        format!(
                            "{}",
                            match self.multiple {
                                Some(m) => format!(
                                    "  {}倍{}",
                                    m,
                                    if self.Additional { "追加" } else { "" }
                                ),
                                None => format!("{}", if self.Additional { "  追加" } else { "" }),
                            }
                        ),
                        keyfront.join(" "),
                        filler_front_result.join(" "),
                        keybehind_result.join(" "),
                        filler_behind_result.join(" ")
                    ));
                }
            }
            if let Some(handle) = handle {
                handle.join().unwrap();
            }
        }
        Ok(result)
    }
    pub fn all_cast_seven() -> Result<Vec<String>, ()> {
        let progress = Arc::new(AtomicUsize::new(0));
        let progress_clone = Arc::clone(&progress);
        let mut first: Vec<String> = (1..=35).map(|num| format!("{:02}", num)).collect();
        let mut last: Vec<String> = (1..=14)
            .map(|num| format!("{:02}", ((num - 1) % 12) + 1))
            .collect();
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
                print!("{}{} {:>3}", bar, space, (percent * 100.0).round() as usize);
                stdout().flush().unwrap();
            }
        });
        for _ in 0..times {
            first.shuffle(&mut rng);
            last.shuffle(&mut rng);
            progress.fetch_add(1, Ordering::Relaxed);
        }
        let mut result = Vec::new();
        for _ in 0..7 {
            let mut front_result: Vec<String> = first.drain(0..5).collect();
            let mut behind_result: Vec<String> = last.drain(0..2).collect();
            front_result.sort();
            behind_result.sort();
            result.push(format!(
                "{} : {}",
                front_result.join(" "),
                behind_result.join(" ")
            ));
        }
        handle.join().unwrap();
        println!();
        Ok(result)
    }
}
#[cfg(test)]
mod test_probability {

    use std::{
        sync::{
            Arc,
            atomic::{AtomicU64, Ordering},
        },
        thread,
    };

    use rand::seq::SliceRandom;

    #[test]
    fn test() {
        let value: u64 = 0b_01001_00010_00010_00000_00000_00000_00100__00010_00010_00;
        let first = vec![2, 5, 9, 14, 33];
        let last = vec![4, 9];
        let mut result_value: u64 = 0;
        for i in 0..5 {
            result_value |= 1 << (47 - first[i]); // 最高位是1号，最低位是35号
        }
        for i in 0..2 {
            result_value |= 1 << (12 - last[i]); // 最高位是1号，最低位是12号
        }
        println!("{:048b}", result_value);
        println!("{:048b}", value);
        assert_eq!(result_value, value);
    }
    #[test]
    fn fast_test() {
        let value: u64 = 0b_01001_00010_00010_00000_00000_00000_00100__00010_00010_00;
        let mut first = (1..=35).collect::<Vec<usize>>();
        let mut last = (1..=12).collect::<Vec<usize>>();
        let mut rng: rand::prelude::ThreadRng = rand::rng();

        let times = Arc::new(AtomicU64::new(0));
        let times_clone = Arc::clone(&times);
        let result_value_arc = Arc::new(AtomicU64::new(0));
        let result_value_clone = Arc::clone(&result_value_arc);
        let handle = thread::spawn(move || {
            loop {
                print!(
                    "\r{} {:048b}",
                    times_clone.load(Ordering::Relaxed),
                    result_value_clone.load(Ordering::Relaxed)
                );
            }
        });

        loop {
            times.fetch_add(1, Ordering::Relaxed);
            first.shuffle(&mut rng);
            last.shuffle(&mut rng);
            let mut result_value: u64 = 0;
            //result_value是37位的二进制数,从高位往低位看,将first前5个数的字面值对应的位置1,将last前两个数的字面值+35对应的位置置1
            for i in 0..5 {
                result_value |= 1 << (47 - first[i]); // 最高位是1号，最低位是35号
            }
            for i in 0..2 {
                result_value |= 1 << (12 - last[i]); // 最高位是1号，最低位是12号
            }
            result_value_arc.store(result_value, Ordering::Relaxed);
            if result_value == value {
                handle.join().unwrap();
                println!("\n{:048b}", result_value);
                println!("{:048b}", value);
                break;
            }
        }
        assert_eq!(value, value);
    }
}
