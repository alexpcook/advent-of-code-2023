use std::fs;

fn differences(v: &Vec<i32>) -> Vec<Vec<i32>> {
    let n = v.len().saturating_sub(1);

    let mut diffs = Vec::with_capacity(n);

    for i in 0..n {
        let current = v.get(i).unwrap();
        let next = v.get(i + 1).unwrap();
        diffs.push(next - current);
    }

    let mut result = vec![diffs.clone()];

    if diffs.iter().any(|&i| i != 0) {
        for d in differences(&diffs) {
            result.push(d);
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input/day9.txt").unwrap();

    let mut sum = 0;

    let histories: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    for history in histories {
        let differences = differences(&history);

        let firsts: Vec<_> = differences
            .into_iter()
            .map(|diffs| diffs.into_iter().nth(0).unwrap())
            .rev()
            .collect();

        println!("{firsts:?}");

        let first = history.first().unwrap();
        let step = firsts.into_iter().reduce(|acc, x| x - acc).unwrap();

        sum += first - step;

        println!("sum={sum} first={first} step={step}");
    }

    println!("day 9, part 1: {sum}");
}
