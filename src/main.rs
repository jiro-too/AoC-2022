mod day1;
mod day2;
mod day3;
mod day4;
mod day6;

use std::{
    fs,
    io::{stdout, Write},
};

fn loadfile(path: &str) -> Option<String> {
    let content = fs::read_to_string(path);
    match content {
        Ok(_) => Some(content.unwrap()),
        Err(_) => None,
    }
}

fn main() {
    println!("DAY 1 SOLUTION: {}", (day1::solve(loadfile("advent1.txt"))));

    println!("\nDay 2 solution(s): ");
    day2::solve(loadfile("advent2.txt"));
    println!(
        "\nDay 3 solution(s):\n\tPart 1 {}\n\tPart 2 {}",
        day3::solve_part1(loadfile("advent3.txt")),
        day3::solve_part2(loadfile("/home/nolife/advent_of_code/advent3.txt"))
    );

    println!(
        "\nDAY 4 SOLUTION: {:?}",
        (day4::solve(loadfile("advent4.txt")))
    );

    println!(
        "\nDAY 6 SOLUTION:\n\tPart 1: {:?}\n\tPart 2: {:?}",
        (day6::solve_part1(loadfile("advent6.txt"))),
        (day6::solve_part2(loadfile("advent6.txt")))
    );
}
