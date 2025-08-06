use crate::sql::sqltable::init_db;

fn get_current_qihao() -> u32 {
    use chrono::{Local, NaiveDate};

    let base_qihao = 25088u32;
    let base_date = NaiveDate::from_ymd_opt(2025, 8, 3).unwrap(); // 2025-08-05
    let today = Local::now().date_naive();

    if today < base_date {
        return 0;
    }

    let diff_days = (today - base_date).num_days();
    let weeks = diff_days / 7;
    let days_left = diff_days % 7;

    // 每周3期
    let mut qihao = base_qihao + (weeks * 3) as u32;
    if days_left >= 1 {
        qihao += 1;
    }
    if days_left >= 4 {
        qihao += 1;
    }
    if days_left >= 6 {
        qihao += 1;
    }

    qihao
}
use itertools::Itertools;
#[allow(non_snake_case)]
fn String2V_String(s: String) -> Vec<String> {
    let s = s.trim();
    let mut result: Vec<String> = Vec::new();

    // 第一种格式：5+2
    if s.matches(':').count() == 1 && s.split_whitespace().count() == 7 {
        result.push(s.to_string());
        result.sort();
        return result;
    }
    // 第二种格式：n+m
    if s.matches(':').count() == 1 {
        let parts: Vec<&str> = s.split(':').collect();
        let front: Vec<&str> = parts[0].split_whitespace().collect();
        let behind: Vec<&str> = parts[1].split_whitespace().collect();
        // 前区n选5，后区m选2
        for f in front.iter().copied().combinations(5) {
            for b in behind.iter().copied().combinations(2) {
                let mut f_sorted = f.clone();
                let mut b_sorted = b.clone();
                f_sorted.sort();
                b_sorted.sort();
                result.push(format!("{} : {}", f_sorted.join(" "), b_sorted.join(" ")));
            }
        }
        result.sort();

        return result;
    }

    if s.contains("前区胆:")
        && s.contains("前区拖:")
        && s.contains("后区胆:")
        && s.contains("后区拖:")
    {
        // 提取各区号码
        let get_nums = |label: &str| -> Vec<&str> {
            s.split(label)
                .nth(1)
                .and_then(|x| x.lines().next())
                .unwrap_or("")
                .split_whitespace()
                .collect()
        };
        let keyfront = get_nums("前区胆:");
        let fillerfront = get_nums("前区拖:");
        let keybehind = get_nums("后区胆:");
        let fillerbehind = get_nums("后区拖:");

        // 前区组合
        let need_front = 5 - keyfront.len();
        let need_behind = 2 - keybehind.len();
        for f in fillerfront.iter().copied().combinations(need_front) {
            let mut front = keyfront.clone();
            front.extend(f);
            front.sort();
            for b in fillerbehind.iter().copied().combinations(need_behind) {
                let mut behind = keybehind.clone();
                behind.extend(b);
                behind.sort();
                result.push(format!("{} : {}", front.join(" "), behind.join(" ")));
            }
        }
        result.sort();

        return result;
    }

    result
}

pub fn insert_number(buy_number: Vec<String>) {
    let conn = init_db("lottoSql.db").expect("Failed to initialize database");
    let qihao = get_current_qihao();

    //遍历所有String调用String2V_String并收集成一个Vec
    let all_numbers: Vec<String> = buy_number.into_iter().flat_map(String2V_String).collect();
    //遍历all_numbers插入数据库
    all_numbers.iter().for_each(|num| {
        let qihao_str = qihao.to_string();
        conn.execute(
            "INSERT INTO numbers (qihao_id, number) VALUES (?1, ?2)",
            &[&qihao_str, num],
        )
        .expect("Failed to insert number");
    });
}
#[allow(non_snake_case)]
fn String2Vecu32(number: String) -> (Vec<u32>, Vec<u32>) {
    let parts: Vec<&str> = number.split(':').collect();
    let front = parts[0].trim();
    let behind = parts[1].trim();

    let mut front_numbers: Vec<u32> = Vec::new();
    let mut behind_numbers: Vec<u32> = Vec::new();

    front.split_whitespace().for_each(|n| {
        if let Ok(num) = n.parse::<u32>() {
            front_numbers.push(num);
        }
    });
    behind.split_whitespace().for_each(|n| {
        if let Ok(num) = n.parse::<u32>() {
            behind_numbers.push(num);
        }
    });
    (front_numbers, behind_numbers)
}
pub fn check_number(qihao: u32, atta_number: String) -> u32 {
    let mut count = 0;
    let (front, behind) = String2Vecu32(atta_number);
    let conn = init_db("lottoSql.db").expect("Failed to initialize database");
    //取出数据库中所有qihao的号码并存入Vec<String>中
    let mut stmt = conn
        .prepare("SELECT number FROM numbers WHERE qihao_id = ?1")
        .expect("Failed to prepare statement");
    let rows = stmt
        .query_map([qihao], |row| row.get::<_, String>(0))
        .expect("Failed to query rows");

    for row in rows {
        let (f, b) = String2Vecu32(row.expect("Failed to get row"));
        let front_count = f.iter().filter(|&n| front.contains(n)).count();
        let behind_count = b.iter().filter(|&n| behind.contains(n)).count();
        match (front_count, behind_count) {
            (5, 2) => count += 1000_0000,
            (5, 1) => count += 30_0000,
            (5, 0) => count += 1_0000,
            (4, 2) => count += 3000,
            (4, 1) => count += 300,
            (3, 2) => count += 200,
            (4, 0) => count += 100,
            (2, 2) => count += 15,
            (3, 1) => count += 15,
            (1, 2) => count += 5,
            (2, 1) => count += 5,
            (0, 2) => count += 5,
            _ => (),
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_String2V_String() {
        let mut right = vec![
            "10 12 18 22 29 : 02 03".to_string(),
            "10 12 18 22 29 : 01 03".to_string(),
            "10 12 18 22 29 : 01 02 ".to_string(),
        ];
        right.sort();
        assert_eq!(
            String2V_String("10 12 18 22 29 : 01 02 03".to_string()),
            vec![
                "10 12 18 22 29 : 02 03".to_string(),
                "10 12 18 22 29 : 01 03".to_string(),
                "10 12 18 22 29 : 01 02 ".to_string()
            ]
        );
    }
    #[test]
    fn insert() {
        insert_number(vec![
            "01 02 03 04 05 : 01 02 03".to_string(),
            "01 02 03 04 05 : 04 05 06".to_string(),
        ]);
    }
    #[test]
    fn check() {
        let count = check_number(25089, "01 02 03 04 05 : 01 02 ".to_string());
        println!("Check Number Count: {}", count);
    }
}

#[cfg(test)]
mod test_data {
    use chrono::NaiveDate;

    pub fn get_current_qihao1(today: NaiveDate) -> u32 {
        let base_qihao = 25089u32;
        let base_date = NaiveDate::from_ymd_opt(2025, 8, 6).unwrap(); // 2025-08-06

        if today < base_date {
            return 0;
        }

        let diff_days = (today - base_date).num_days();
        let weeks = diff_days / 7;
        let days_left = diff_days % 7;

        // 每周3期
        let mut qihao = base_qihao + (weeks * 3) as u32;
        if days_left >= 1 {
            qihao += 1;
        }
        if days_left >= 4 {
            qihao += 1;
        }
        if days_left >= 6 {
            qihao += 1;
        }

        qihao
    }

    #[test]
    fn test8_6() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 6).unwrap()) == 25089);
    }
    #[test]
    fn test8_7() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 7).unwrap()) == 25090);
    }
    #[test]
    fn test8_8() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 8).unwrap()) == 25090);
    }
    #[test]
    fn test8_9() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 9).unwrap()) == 25090);
    }
    #[test]
    fn test8_10() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 10).unwrap()) == 25091);
    }
    #[test]
    fn test8_11() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 11).unwrap()) == 25091);
    }
    #[test]
    fn test8_12() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 12).unwrap()) == 25092);
    }
    #[test]
    fn test8_13() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 13).unwrap()) == 25092);
    }
    #[test]
    fn test8_14() {
        assert!(get_current_qihao1(NaiveDate::from_ymd_opt(2025, 8, 14).unwrap()) == 25093);
    }
}
