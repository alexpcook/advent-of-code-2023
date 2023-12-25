use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/day8.txt").unwrap();

    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let nodes: HashMap<_, _> = nodes
        .lines()
        .map(|line| {
            let node = line.get(0..3).unwrap();
            let left = line.get(7..10).unwrap();
            let right = line.get(12..15).unwrap();

            (node, (left, right))
        })
        .collect();

    let mut node = nodes.get(&"AAA").unwrap();
    let mut steps = 0;

    let mut starting_nodes: Vec<_> = nodes
        .keys()
        .filter_map(|&k| {
            if k.ends_with('A') {
                Some(nodes.get(k).unwrap())
            } else {
                None
            }
        })
        .collect();
    println!("{starting_nodes:?}");

    for direction in directions.chars().cycle() {
        steps += 1;

        let next = if direction == 'L' { node.0 } else { node.1 };

        if next == "ZZZ" {
            break;
        }

        node = nodes.get(next).unwrap();
    }

    println!("day 8, part 1: {steps}");

    steps = 0;

    for direction in directions.chars().cycle() {
        steps += 1;
        let mut nexts = Vec::with_capacity(starting_nodes.len());

        for node in starting_nodes.iter_mut() {
            let next = if direction == 'L' { node.0 } else { node.1 };
            nexts.push(next);
            *node = nodes.get(next).unwrap()
        }

        for (i, next) in nexts.into_iter().enumerate() {
            if next.ends_with('Z') {
                println!("{i} = {steps}");
            }
        }

        if steps > 1_000_000 {
            return;
        }
    }

    println!("day 8, part 2: {steps}");
}
