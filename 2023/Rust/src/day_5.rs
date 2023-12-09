use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
pub struct Map {
    dest_range: i64,
    source_range: i64,
    range_length: i64,
}

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item = String> {
    let path = if test_mode {
        format!("inputs_test/day_5_{}.in", part)
    } else {
        "inputs/day_5.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

fn parse_seeds(line: &str) -> Vec<i64> {
    line.replace("seeds: ", "")
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_map(line: &str) -> Map {
    let params = line
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let dest_range = params[0];
    let source_range = params[1];
    let range_length = params[2];

    Map {
        dest_range,
        source_range,
        range_length,
    }
}

pub fn process_seed(seeds: Vec<i64>, maps: &[Map]) -> Vec<i64> {
    let mut new_locations = vec![];
    for seed in seeds {
        let mut new_location: i64 = -1;
        for map in maps.iter() {
            if seed > map.source_range && seed < map.source_range + map.range_length {
                new_location = seed + (map.dest_range - map.source_range)
            }
        }
        if new_location == -1 {
            new_location = seed;
        }
        new_locations.push(new_location);
    }
    new_locations
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let mut lines = parse_input(test_mode, 1);
    let mut seeds = parse_seeds(lines.next().unwrap().as_str());

    let mut almanac_step: Vec<Map> = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.ends_with(':') {
            if !almanac_step.is_empty() {
                seeds = process_seed(seeds, &almanac_step);
            }
            almanac_step = vec![];
        } else {
            almanac_step.push(parse_map(&line));
        }
    }
    seeds = process_seed(seeds, &almanac_step);
    seeds.into_iter().min().unwrap() as usize
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let mut lines = parse_input(test_mode, 2);

    let base_seeds = parse_seeds(lines.next().unwrap().as_str());
    let seeds_ranges = base_seeds.chunks(2).map(|sr| (sr[0], sr[1])).collect::<Vec<(i64, i64)>>();

    let mut mins= vec![];

    for range in seeds_ranges {


        let mut lines = parse_input(test_mode, 2);
        lines.next();

        let mut seeds: Vec<i64> = (range.0..(range.0 + range.1 - 1)).collect();

        let mut almanac_step: Vec<Map> = vec![];
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if line.ends_with(':') {
                if !almanac_step.is_empty() {
                    seeds = process_seed(seeds, &almanac_step);
                }
                almanac_step = vec![];
            } else {
                almanac_step.push(parse_map(&line));
            }
        }
        seeds = process_seed(seeds, &almanac_step);

        mins.push(seeds.into_iter().min().unwrap() as usize);
        println!("{:?}", range);
    }

    *mins.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_5::{process_seed, solve_part_1, solve_part_2, Map};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 35);
    }

    #[test]
    fn should_parse_seed() {
        let map_2 = Map {
            dest_range: 50,
            source_range: 98,
            range_length: 2,
        };
        let map_1 = Map {
            dest_range: 52,
            source_range: 50,
            range_length: 48,
        };
        let result = process_seed(vec![79, 14, 55, 13], &[map_2, map_1]);
        assert_eq!(result, vec![81, 14, 57, 13]);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 46);
    }
}
