use std::fs;

fn main() {
    let input = fs::read_to_string("input/day5.txt").unwrap();

    let mut blocks = input.split("\n\n");

    let mut seeds_iter = blocks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    let mut seeds = vec![];

    while let (Some(start), Some(length)) = (seeds_iter.next(), seeds_iter.next()) {
        seeds.push((start, length));
    }

    let mut maps: Vec<Vec<_>> = blocks
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|s| {
                    let mut numbers = s.split_whitespace();
                    let destination_start = numbers.next().unwrap().parse::<u64>().unwrap();
                    let source_start = numbers.next().unwrap().parse::<u64>().unwrap();
                    let length = numbers.next().unwrap().parse::<u64>().unwrap();
                    (destination_start, source_start, length)
                })
                .collect()
        })
        .collect();

    maps.reverse();

    let mut location = 0;

    loop {
        location += 1;
        let mut result = location;

        for map in &maps {
            for &(dest, source, length) in map {
                if result >= dest && result < dest + length {
                    result = source + result - dest;
                    break;
                }
            }
        }

        for &(start, length) in &seeds {
            // if result == start || result == length {
            //     println!("lowest location, part 1: {location}");
            //     return;
            // }
            if result >= start && result < start + length {
                println!("lowest location, part 2: {location}");
                return;
            }
        }
    }
}
