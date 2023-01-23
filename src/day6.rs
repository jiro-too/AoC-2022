use itertools::Itertools;
use std::borrow::BorrowMut;
struct ReverseIter {
    slice: &'static [u8],
    pos: isize,
}

fn reverse_iterator(item: &'static [u8]) -> ReverseIter {
    return ReverseIter {
        slice: item,
        pos: (item.len() - 1) as isize,
    };
}

pub fn solve_part1(buf: Option<String>) -> u64 {
    let mut input: String = String::new();
    if let Some(str) = buf {
        input = str;
    } else {
        println!("Unable to load the file\n");
    }

    // input has one line

    let byte_array = input.as_bytes();
    byte_array
        .windows(4)
        .position(|word| {
            let mut arr = [0u8; 4];
            let mut index = 0;
            for x in word {
                for i in 0..index {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[index] = *x;
                index += 1;
            }
            return true;
        })
        .map(|x| x + 4)
        .unwrap() as u64
}

pub fn solve_part2(buf: Option<String>) -> u64 {
    let mut input: String = String::new();
    if let Some(str) = buf {
        input = str;
    } else {
        println!("Unable to load the file\n");
    }

    // input has one line

    let byte_array = input.as_bytes();
    byte_array
        .windows(14)
        .position(|word| {
            let mut arr = [0u8; 14];
            let mut index = 0;
            for x in word {
                for i in 0..index {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[index] = *x;
                index += 1;
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap() as u64
}
