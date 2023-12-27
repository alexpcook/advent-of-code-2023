use std::{collections::HashSet, fs};

fn s_position(pipes: &Vec<Vec<char>>) -> (usize, usize) {
    let mut s_position = None;

    'OUTER: for (y, row) in pipes.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                s_position = Some((x, y));
                break 'OUTER;
            }
        }
    }

    s_position.unwrap()
}

fn get_char((x, y): (usize, usize), pipes: &Vec<Vec<char>>) -> char {
    *pipes.get(y).unwrap().get(x).unwrap()
}

fn can_go_north(c: &char) -> bool {
    ['|', 'L', 'J', 'S'].contains(c)
}

fn can_go_south(c: &char) -> bool {
    ['|', '7', 'F', 'S'].contains(c)
}

fn can_go_east(c: &char) -> bool {
    ['-', 'L', 'F', 'S'].contains(c)
}

fn can_go_west(c: &char) -> bool {
    ['-', 'J', '7', 'S'].contains(c)
}

fn next_square(
    (x, y): (usize, usize),
    pipes: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    if !visited.insert((x, y)) {
        return None;
    }

    let current_char = get_char((x, y), pipes);

    if x > 0 && can_go_west(&current_char) {
        let next_char = get_char((x - 1, y), pipes);
        if can_go_east(&next_char) && !visited.contains(&(x - 1, y)) {
            return Some((x - 1, y));
        }
    }

    if x < pipes.get(0).unwrap().len() - 1 && can_go_east(&current_char) {
        let next_char = get_char((x + 1, y), pipes);
        if can_go_west(&next_char) && !visited.contains(&(x + 1, y)) {
            return Some((x + 1, y));
        }
    }

    if y > 0 && can_go_north(&current_char) {
        let next_char = get_char((x, y - 1), pipes);
        if can_go_south(&next_char) && !visited.contains(&(x, y - 1)) {
            return Some((x, y - 1));
        }
    }

    if y < pipes.len() - 1 && can_go_south(&current_char) {
        let next_char = get_char((x, y + 1), pipes);
        if can_go_north(&next_char) && !visited.contains(&(x, y + 1)) {
            return Some((x, y + 1));
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input/day10.txt").unwrap();

    let pipes: Vec<Vec<_>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut position = s_position(&pipes);
    let mut visited = HashSet::new();
    let mut steps = 0;

    while let Some(next) = next_square(position, &pipes, &mut visited) {
        steps += 1;
        position = next;
    }

    let steps = if steps % 2 == 0 {
        steps / 2
    } else {
        steps / 2 + 1
    };

    println!("day 10, part 1: {steps}");
}
