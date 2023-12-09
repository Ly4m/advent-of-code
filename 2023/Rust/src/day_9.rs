use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

fn parse_input(test_mode: bool) -> impl Iterator<Item=String> {
    const TEST_INPUT_PATH: &str = "inputs_test/";
    const INPUT_PATH: &str = "inputs/";

    let path = format!("{}day_9.in", if test_mode { TEST_INPUT_PATH } else { INPUT_PATH });
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

pub fn predict_next_value(suite: &[isize]) -> isize {
    if suite.iter().all(|x| *x == 0) {
        return 0;
    }
    let differences: Vec<isize> = suite
        .iter()
        .enumerate()
        .skip(1)
        .map(|x| x.1 - suite[x.0 - 1])
        .collect();

    differences
        .last()
        .unwrap()
        .add(predict_next_value(&differences))
}

fn parse_suite(line: String) -> Vec<isize>{
    line.split_whitespace()
        .map(|number| number.parse::<isize>().expect("Invalid number"))
        .collect::<Vec<isize>>()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    parse_input(test_mode)
        .map(parse_suite)
        .map(|x| predict_next_value(&x).add(x.last().unwrap()))
        .map(|x| x as usize)
        .sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let sum: isize = parse_input(test_mode)
        .map(parse_suite)
        .map(|mut x| {
            x.reverse();
            predict_next_value(&x).add(x.last().unwrap())
        })
        .sum();
    sum as usize
}

#[cfg(test)]
mod tests {
    use crate::day_9::{predict_next_value, solve_part_1, solve_part_2};

    #[test]
    fn should_predict_next_number_zeroes() {
        let result = predict_next_value(&[0, 0, 0, 0, 0, 0]);
        assert_eq!(result, 0);
    }

    #[test]
    fn should_predict_next_number() {
        let result = predict_next_value(&[0, 3, 6, 9, 12, 15]);
        assert_eq!(result, 3);
    }

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 114);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 2);
    }
}
