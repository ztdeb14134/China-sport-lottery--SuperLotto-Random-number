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
                            "{}{}",
                            if self.multiple.is_some() {
                                format!("{}倍", self.multiple.unwrap())
                            } else {
                                "".to_string()
                            },
                            if self.Additional { "追加" } else { "" }
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
                            "{}{}",
                            if self.multiple.is_some() {
                                format!("{}倍", self.multiple.unwrap())
                            } else {
                                "".to_string()
                            },
                            if self.Additional { "追加" } else { "" }
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
                            "{}{}",
                            if self.multiple.is_some() {
                                format!("{}倍", self.multiple.unwrap())
                            } else {
                                "".to_string()
                            },
                            if self.Additional { "追加" } else { "" }
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
}
