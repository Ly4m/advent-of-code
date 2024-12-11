use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::read_to_string;

fn parse_input(test_mode: bool, part: i8) -> Vec<usize> {
    let path = if test_mode {
        format!("inputs_test/day_11_{}.in", part)
    } else {
        "inputs/day_11.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    read_to_string(reader)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn blink_with_memory(pebbles_count: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_pebbles: HashMap<usize, usize> = HashMap::new();
    pebbles_count.iter().for_each(|(key, value)| {
        if *key == 0 {
            *new_pebbles.entry(1).or_insert(0) += value;
        } else {
            let string_representation = key.to_string();
            if string_representation.len() % 2 == 0 {
                let string = string_representation.split_at(string_representation.len() / 2);
                *new_pebbles.entry(string.0.parse().unwrap()).or_insert(0) += value;
                *new_pebbles.entry(string.1.parse().unwrap()).or_insert(0) += value;
            } else {
                *new_pebbles.entry(key * 2024).or_insert(0) += value;
            }
        }
    });
    new_pebbles
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let input = parse_input(test_mode, 1);
    let mut pebbles_count: HashMap<usize, usize> = HashMap::new();
    for x in input {
        *pebbles_count.entry(x).or_insert(0) += 1;
    }
    for _ in 0..25 {
        pebbles_count = blink_with_memory(pebbles_count);
    }
    pebbles_count.values().sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let input = parse_input(test_mode, 1);
    let mut pebbles_count: HashMap<usize, usize> = HashMap::new();
    for x in input {
        *pebbles_count.entry(x).or_insert(0) += 1;
    }
    for _ in 0..75 {
        pebbles_count = blink_with_memory(pebbles_count);
    }
    pebbles_count.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_11::{solve_part_1};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 55312);
    }
}
