use std::collections::HashSet;
use std::fs;

fn parse_input() -> Vec<usize> {
    fs::read_to_string("inputs/day_9.in").unwrap().lines()
        .map(|x| x.parse::<usize>().expect("Not a number found"))
        .collect()
}

pub fn solve_part_2() -> usize {
    let numbers = parse_input();
    let target = 556543474;

    for (index, number) in numbers.iter().enumerate() {
        let mut current_streak = HashSet::new();

        let mut aggregator = *number;

        for inner_number in numbers.iter().skip(index + 1) {
            if aggregator + inner_number > target {
                current_streak.drain();
                break;
            }

            if aggregator + inner_number == target {
                current_streak.insert(inner_number);
                println!("Found sum {} for {:?}", target, current_streak);
                return *current_streak.iter().min().unwrap() + *current_streak.iter().max().unwrap();
            }

            current_streak.insert(inner_number);
            aggregator += *inner_number;
        }
    }

    0
}

pub fn solve_part_1() -> usize {
    let preamble_size = 25;
    let lines = parse_input();

    for (index, number) in lines.iter().enumerate().skip(preamble_size) {
        if !two_sum_exists(&lines.iter().skip(index - preamble_size).take(preamble_size)
            .collect::<Vec<&usize>>(), number) {
            return *number;
        }
    };
    0
}

fn two_sum_exists(report: &Vec<&usize>, &year: &usize) -> bool {
    for (index, value) in report.iter().enumerate() {
        for value2 in report[index..report.len()].iter() {
            if *value + *value2 == year {
                return true;
            }
        }
    }
    false
}
