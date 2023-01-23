fn convert_input(digits: &str, to_rem: &str) -> u64 {
    return digits.strip_prefix(to_rem).unwrap().parse::<u64>().unwrap();
}
use std::ops::Range;

fn range(a: &(u64, u64), b: &(u64, u64)) -> bool {
    return a.0 <= b.0 && a.1 >= b.1;
}
fn parse(input: &str) -> Vec<(std::ops::Range<u8>, std::ops::Range<u8>)> {
    input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(',')?;
            let (a_begin, a_end) = left.split_once('-')?;
            let (b_begin, b_end) = right.split_once('-')?;
            Some((
                (a_begin.parse().ok()?)..(a_end.parse().ok()?),
                (b_begin.parse().ok()?)..(b_end.parse().ok()?),
            ))
        })
        .collect()
}

fn in_range(a: &Range<u8>, b: &Range<u8>) -> bool {
    b.start >= a.start && b.end <= a.end
}

pub fn solve(buf: Option<String>) -> i128 {
    let mut input: String = String::new();
    if let Some(str) = buf {
        input = str;
    } else {
        println!("Unable to load the file\n");
    }

    let mut total:i128 = 0;
    input
        .lines()
        .for_each(|line| 
            {
                let count = 
                parse(line)
                .iter()
                .filter(|(a, b)| in_range(a, b) || in_range(b, a)).count();

                total+=count as i128;
            });

    total
}
