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

fn is_in_loop((x, y): (usize, usize), loop_squares: &HashSet<(usize, usize)>) -> bool {
    loop_squares.contains(&(x, y))
}

fn get_area(
    (x, y): (usize, usize),
    pipes: &Vec<Vec<char>>,
    loop_squares: &HashSet<(usize, usize)>,
    traversed: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    traversed.insert((x, y));

    let mut result = HashSet::new();
    result.insert((x, y));

    if x > 0 && !loop_squares.contains(&(x - 1, y)) && !traversed.contains(&(x - 1, y)) {
        result.insert((x - 1, y));
        result.extend(get_area((x - 1, y), pipes, loop_squares, traversed));
    }

    if x < pipes.get(0).unwrap().len() - 1
        && !loop_squares.contains(&(x + 1, y))
        && !traversed.contains(&(x + 1, y))
    {
        result.insert((x + 1, y));
        result.extend(get_area((x + 1, y), pipes, loop_squares, traversed));
    }

    if y > 0 && !loop_squares.contains(&(x, y - 1)) && !traversed.contains(&(x, y - 1)) {
        result.insert((x, y - 1));
        result.extend(get_area((x, y - 1), pipes, loop_squares, traversed));
    }

    if y < pipes.len() - 1
        && !loop_squares.contains(&(x, y + 1))
        && !traversed.contains(&(x, y + 1))
    {
        result.insert((x, y + 1));
        result.extend(get_area((x, y + 1), pipes, loop_squares, traversed));
    }

    result
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

    let mut areas = vec![];
    let mut counted: HashSet<(usize, usize)> =
        HashSet::with_capacity(pipes.len() * pipes.get(0).unwrap().len() - visited.len());

    for (y, row) in pipes.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if !is_in_loop((x, y), &visited) && !counted.contains(&(x, y)) {
                areas.push(get_area((x, y), &pipes, &visited, &mut counted));
            }
        }
    }

    println!("got {} areas", areas.len());
    println!("{areas:?}");
}
