use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> (HashSet<String>, Vec<String>) {
    let path = if test_mode {
        format!("inputs_test/day_19_{}.in", part)
    } else {
        "inputs/day_19.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let available_patterns: HashSet<String> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let _ = lines.next().unwrap();

    let requested_patterns: Vec<String> = lines.map(|l| l.unwrap()).collect();

    (available_patterns, requested_patterns)
}

fn is_design_possible(towels: &HashSet<String>, design: &str) -> bool {
    let design_length = design.len();
    let mut possible_designs = vec![false; design_length + 1];
    possible_designs[0] = true;

    for start_index in 0..=design.len() {
        if possible_designs[start_index] {
            for towel in towels {
                if design[start_index..].starts_with(towel) {
                    possible_designs[start_index + towel.len()] = true;
                }
            }
        }
    }

    possible_designs[design_length]
}

fn count_design_possibilities(
    towels: &HashSet<String>,
    design: &str,
    found_possibilities: &mut HashMap<String, usize>,
    max_len: usize,
) -> usize {
    let mut counter = 0;

    if found_possibilities.contains_key(design) {
        return *found_possibilities.get(design).unwrap();
    }
    if design.is_empty() {
        return 1;
    }

    for i in 1..=max_len.min(design.len()) {
        if towels.contains(&design[..i]) {
            counter += count_design_possibilities(towels, &design[i..], found_possibilities, max_len);
        }
    }
    found_possibilities.insert(design.to_string(), counter);

    counter
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let (available_patterns, requested_patterns) = parse_input(test_mode, 1);

    requested_patterns
        .iter()
        .filter(|p| is_design_possible(&available_patterns, p))
        .count()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let (available_patterns, requested_patterns) = parse_input(test_mode, 1);
    let mut found_design: HashMap<String, usize> = HashMap::new();
    let max_pattern_len = available_patterns.iter().map(|v| v.len()).max().unwrap();
    requested_patterns
        .iter()
        .map(|p| count_design_possibilities(&available_patterns, p, &mut found_design, max_pattern_len))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_19::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 16);
    }
}
