fn convert_input(digits: &str, to_rem: &str) -> u64 {
    return digits.strip_prefix(to_rem).unwrap().parse::<u64>().unwrap();
}
use std::ops::Range;

fn range(a: &(u64, u64), b: &(u64, u64)) -> bool {
    return a.0 <= b.0 && a.1 >= b.1;
}
fn parse(input: &str) -> Vec<(std::ops::Range<u64>, std::ops::Range<u64>)> {
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

fn in_range(a: &Range<u64>, b: &Range<u64>) -> bool {
    b.start >= a.start && b.end <= a.end
}

fn overlaps(a: &Range<u64>, b: &Range<u64>) -> bool {
    (if a.start > b.start { a.start } else { b.start })
        <= (if a.end < b.end { a.end } else { b.end })
}
pub fn solve(buf: Option<String>) -> (i128, i128) {
    let mut input: String = String::new();
    if let Some(str) = buf {
        input = str;
    } else {
        println!("Unable to load the file\n");
    }

    let mut total_part1: i128 = 0;
    let mut total_part2: i128 = 0;
    input.lines().for_each(|line| {
        let count = parse(line)
            .iter()
            .filter(|(a, b)| in_range(a, b) || in_range(b, a))
            .count();

        total_part1 += count as i128;
    });

    input.lines().for_each(|line| {
        let count = parse(line)
            .iter()
            .filter(|(a, b)| overlaps(a, b) || overlaps(b, a))
            .count();

        total_part2 += count as i128;
    });

    (total_part1, total_part2)
}
