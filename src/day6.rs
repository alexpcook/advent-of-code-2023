pub fn solve(input: String) {
    let mut race_durations_and_distances: Vec<_> = input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let race_distances = race_durations_and_distances.pop().unwrap();
    let race_durations = race_durations_and_distances.pop().unwrap();

    let mut race_distance_part2 = String::new();
    let mut race_duration_part2 = String::new();

    for (race_duration, race_distance) in race_durations.iter().zip(race_distances.iter()) {
        race_duration_part2.push_str(race_duration.to_string().as_str());
        race_distance_part2.push_str(race_distance.to_string().as_str());
    }

    let race_distance_part2 = race_distance_part2.parse::<u64>().unwrap();
    let race_duration_part2 = race_duration_part2.parse::<u64>().unwrap();

    let races_iter = race_durations.into_iter().zip(race_distances);

    let mut number_of_ways_to_win_multiplied = 1;

    for (race_duration, race_record_distance) in races_iter {
        let mut number_of_ways_to_win = 0;

        // Charging boat for no duration or entire race duration will never win
        for duration in 1..race_duration {
            // 1 mm/ms per ms of charging
            let boat_speed = duration;

            let remaining_race_duration = race_duration - boat_speed;

            let boat_distance = boat_speed * remaining_race_duration;

            if boat_distance > race_record_distance {
                number_of_ways_to_win += 1;
            }
        }

        number_of_ways_to_win_multiplied *= number_of_ways_to_win;
    }

    println!("part 1: {number_of_ways_to_win_multiplied}");

    let mut number_of_ways_to_win = 0;

    for duration in 1..race_duration_part2 {
        let boat_speed = duration;

        let remaining_race_duration = race_duration_part2 - boat_speed;

        let boat_distance = boat_speed * remaining_race_duration;

        if boat_distance > race_distance_part2 {
            number_of_ways_to_win += 1;
        }
    }

    println!("part 2: {number_of_ways_to_win}");
}
