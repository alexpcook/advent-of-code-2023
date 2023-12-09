use std::fs;

#[derive(Debug, Default)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Range {
    fn destination(&self, source: u64) -> Option<u64> {
        if source < self.source_start || source > self.source_start + self.length - 1 {
            None
        } else {
            let offset = source - self.source_start;
            Some(self.destination_start + offset)
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn destination(&self, source: u64) -> u64 {
        println!("Map::destination");
        for range in &self.ranges {
            if let Some(destination) = range.destination(source) {
                return destination;
            }
        }
        source
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

impl Almanac {
    fn locations_for_seeds(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|&seed| self.seed_to_soil_map.destination(seed))
            .map(|soil| self.soil_to_fertilizer_map.destination(soil))
            .map(|fertilizer| self.fertilizer_to_water_map.destination(fertilizer))
            .map(|water| self.water_to_light_map.destination(water))
            .map(|light| self.light_to_temperature_map.destination(light))
            .map(|temperature| self.temperature_to_humidity_map.destination(temperature))
            .map(|humidity| self.humidity_to_location_map.destination(humidity))
            .collect()
    }
}

impl From<String> for Almanac {
    fn from(value: String) -> Self {
        let mut blocks = value.split("\n\n").map(|s| s.trim());

        let seeds_block = blocks.next().unwrap();
        // let seeds: Vec<_> = seeds_block
        //     .strip_prefix("seeds: ")
        //     .map(|s| s.split_whitespace().map(|s| s.parse::<u64>().unwrap()))
        //     .unwrap()
        //     .collect();

        let mut seeds_block = seeds_block
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        let mut seeds = vec![];

        while let (Some(start), Some(length)) = (seeds_block.next(), seeds_block.next()) {
            for i in start..start + length {
                seeds.push(i);
            }
        }
        println!("{}", seeds.len());

        let mut almanac = Almanac {
            seeds,
            ..Default::default()
        };

        let maps = vec![
            &mut almanac.seed_to_soil_map,
            &mut almanac.soil_to_fertilizer_map,
            &mut almanac.fertilizer_to_water_map,
            &mut almanac.water_to_light_map,
            &mut almanac.light_to_temperature_map,
            &mut almanac.temperature_to_humidity_map,
            &mut almanac.humidity_to_location_map,
        ];

        for map in maps {
            let block = blocks.next().unwrap();

            let lines = block.lines();

            for (i, line) in lines.enumerate() {
                if i > 0 {
                    let mut split = line.split_whitespace();

                    map.ranges.push(Range {
                        destination_start: split.next().unwrap().parse().unwrap(),
                        source_start: split.next().unwrap().parse().unwrap(),
                        length: split.next().unwrap().parse().unwrap(),
                    });
                }
            }
        }

        almanac
    }
}

fn main() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let almanac = Almanac::from(input);
    let mut locations = almanac.locations_for_seeds();
    locations.sort_unstable();
    println!("{}", locations.first().unwrap());
}
