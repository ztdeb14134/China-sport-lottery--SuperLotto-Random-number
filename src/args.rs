use crate::{printtrait::PrintResult, China_sports_lottery::{PlayType, SuperLotto}};

pub fn args_handle(args: Vec<String>) -> bool {
    if args.len() > 1 {
        if args[1] == "test" || args[1] == "-test" || args[1] == "--test" || args[1] == "-t" {
            SuperLotto::fast_test();
            return true;
        }
        if args[1] == "duplex" || args[1] == "-d" || args[1] == "--d" || args[1] == "--duplex" {
            let front = args[2].parse::<usize>().unwrap_or(0);
            let back = args[3].parse::<usize>().unwrap_or(0);
            let mut super_lotto = SuperLotto::new(PlayType::Duplex(front, back), 1, false);
            super_lotto.draw().printout();
            return true;
        }
        if args[1] == "keyfiller" || args[1] == "-k" || args[1] == "--k" || args[1] == "--keyfiller"
        {
            let key_front = args[2].parse::<usize>().unwrap_or(0);
            let filler_front = args[3].parse::<usize>().unwrap_or(0);
            let key_behind = args[4].parse::<usize>().unwrap_or(0);
            let filler_behind = args[5].parse::<usize>().unwrap_or(0);
            let mut super_lotto = SuperLotto::new(
                PlayType::KeyFiller(key_front, filler_front, key_behind, filler_behind),
                1,
                false,
            );
            super_lotto.draw().printout();
            return true;
        }
        if args[1] == "single" || args[1] == "-s" || args[1] == "--s" || args[1] == "--single" {
            let mut super_lotto = SuperLotto::new(PlayType::Single, 1, false);
            if args.len() > 2 {
                let multiple = args[2].parse::<usize>().unwrap_or(0);
                super_lotto.set_multiple(multiple);
            }
            super_lotto.draw().printout();
            return true;
        }
        if args[1] == "allseven" || args[1] == "--allseven" || args[1] == "-a" || args[1] == "--a" {
            SuperLotto::all_cast_seven().printout();
            return true;
        }
        if args[1] == "birthday" || args[1] == "--birthday" || args[1] == "--b" {
            let mut str = String::new();
            let age: usize;
            if args.len() > 2 {
                str = args[2].clone();
                age = str.parse::<usize>().unwrap_or(0);
            } else {
                println!("请输入年龄:");
                std::io::stdin().read_line(&mut str).unwrap();
                str = str.trim().to_string();
                age = str.parse::<usize>().unwrap_or(0);
            }
            let mut super_lotto = SuperLotto::new(PlayType::Single, 1, true);
            super_lotto.set_multiple(age);
            super_lotto.draw().printout();
            return false;
        } else {
            println!("无效的参数: {}", args[1]);
            return true;
        }
    } else {
        false
    }
}
