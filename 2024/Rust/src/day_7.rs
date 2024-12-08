use io::BufReader;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn parse_input(test_mode: bool, part: i8) -> Vec<(usize, Vec<usize>)> {
    let path = if test_mode {
        format!("inputs_test/day_7_{}.in", part)
    } else {
        "inputs/day_7.in".to_owned()
    };

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let parts: Vec<&str> = line.split(':').map(str::trim).collect();
            let result = usize::from_str(parts[0]).unwrap();
            let values = parts[1]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (result, values)
        })
        .collect()
}
fn test_calibration(result: usize, current_value: usize, values: &[usize]) -> bool {
    if current_value > result {
        return false;
    }
    if values.is_empty() {
        return current_value == result;
    }
    test_calibration(result, current_value + values[0], &values[1..])
        || test_calibration(result, current_value * values[0], &values[1..])
}

fn test_calibration_with_concatenation(
    result: usize,
    current_value: usize,
    values: &[usize],
) -> bool {
    if current_value > result {
        return false;
    }
    if values.is_empty() {
        return current_value == result;
    }
    test_calibration_with_concatenation(result, current_value + values[0], &values[1..])
        || test_calibration_with_concatenation(result, current_value * values[0], &values[1..])
        || test_calibration_with_concatenation(
            result,
            usize::from_str(format!("{}{}", current_value, values[0]).as_str()).unwrap(),
            &values[1..],
        )
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let calibrations = parse_input(test_mode, 1);

    calibrations
        .iter()
        .filter_map(|(result, values)| {
            if test_calibration(*result, values[0], &values[1..]) {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let calibrations = parse_input(test_mode, 1);

    calibrations
        .iter()
        .filter_map(|(result, values)| {
            if test_calibration_with_concatenation(*result, values[0], &values[1..]) {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_7::{solve_part_1, solve_part_2, test_calibration};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 3749);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_calibration_1() {
        let result = test_calibration(190, 10, &vec![19]);
        assert_eq!(result, true);
    }
    #[test]
    fn test_calibration_two_position() {
        let result = test_calibration(3267, 81, &vec![81, 40, 27]);
        assert_eq!(result, true);
    }
}
