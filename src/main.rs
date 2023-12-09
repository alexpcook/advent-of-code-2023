use std::{env, fs};

use advent_of_code_2023::day6;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("want exactly one arg for day to solve");
    }

    let day = match args.get(1).unwrap().parse::<u32>() {
        Ok(n) => n,
        Err(e) => panic!("failed to parse day argument into u32: {e}"),
    };

    let input = match fs::read_to_string(format!("input/day{day}.txt")) {
        Ok(s) => s,
        Err(e) => panic!("failed to read day {day} input: {e}"),
    };

    match day {
        6 => day6::solve(input),
        _ => panic!("failed to find solution for day {day}"),
    }
}
