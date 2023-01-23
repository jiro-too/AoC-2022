use std::collections::HashSet;

fn return_common(s1: &str, s2: &str) -> Option<String> {
    let str: String = String::new();
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return Some(c1.to_string());
            }
        }
    }
    None
}

pub fn solve_part2(buf: Option<String>) -> i64 {
    let mut input: String = String::new();
    if let Some(str) = buf {
        input = str;
    } else {
        println!("Unable to load the file\n");
    }
    let mut total: i64 = 0;
    let mut matching_char: char = '0';
    let mut flag: bool = false;
    let mut lines = input.lines();

    loop {
        let line1 = match lines.next() {
            Some(line) => line,
            None => break,
        };
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();

        for c1 in line1.chars() {
            for c2 in line2.chars() {
                for c3 in line3.chars() {
                    if c1 == c2 && c2 == c3 {
                        flag = true;
                        matching_char = c1;
                    }
                }
                if flag {
                    continue;
                }
            }
            if flag {
                continue;
            }
        }
        if flag {
            let priority = match matching_char {
                'a'..='z' => matching_char as u8 - 'a' as u8 + 1,
                'A'..='Z' => matching_char as u8 - 'A' as u8 + 27,
                _ => 0,
            } as i32;
            total += priority as i64;
        } else {
            // Quit with error
            panic!("No character found that was in all 3 lines");
        }
    }
    total
}

pub fn solve_part1(buf: Option<String>) -> i64 {
    let mut input: String = String::new();
    if let Some(str) = buf {
        input = str;
    } else {
        println!("Unable to load the file\n");
    }
    let mut total: i64 = 0;

    input.lines().for_each(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        let dupls = return_common(left, right).unwrap();

        dupls.chars().for_each(|key| {
            let is_upper = key.is_uppercase();
            if is_upper {
                let priority: i64 = (key as i8 - b'A' as i8) as i64;
                total += priority + 27;
                // println!("{key} ASCII = {}\tA ASCII={}\tPRIORITY={}",key as u8,b'a' as u8,priority);
            } else {
                let priority: i64 = (key as i8 - b'a' as i8) as i64;
                total += priority + 1;
                // println!("{key} ASCII = {}\tA ASCII={}\tPRIORITY={}",key as u8,b'a' as u8,priority);
            }
        });
    });

    total
}
