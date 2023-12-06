use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item=String> {
    let path = if test_mode {
        format!("inputs_test/day_6_{}.in", part)
    } else {
        "inputs/day_6.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

fn parse_races(input: Vec<String>) -> Vec<[u32; 2]> {
    let times = input[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let distances = input[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(a, b)| [a, b])
        .collect()
}

fn parse_big_race(input: Vec<String>) -> (u64, u64) {
    let time = input[0]
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .trim()
        .parse::<u64>().unwrap();

    let distance = input[1]
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .trim()
        .parse::<u64>().unwrap();

    (time, distance)
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines = parse_input(test_mode, 1).collect();
    let races = parse_races(lines);

    races
        .iter()
        .map(|&[time, distance]| {
            (0..time)
                .filter_map(|speed| {
                    if speed * (time - speed) > distance {
                        Some(1)
                    } else {
                        None
                    }
                })
                .count()
        })
        .fold(1, |acc, result| acc * result)
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let lines = parse_input(test_mode, 1).collect();
    let (time, distance) = parse_big_race(lines);
    let mut sum: u64  = 0;


    for speed in 0..time {
        if speed * (time - speed) > distance {
            sum += 1;
        }
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use crate::day_6::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 288);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 71503);
    }
}
