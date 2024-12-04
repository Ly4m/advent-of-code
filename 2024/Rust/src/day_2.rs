use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> Vec<Vec<i32>> {
    let path = if test_mode {
        format!("inputs_test/day_2_{}.in", part)
    } else {
        "inputs/day_2.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let report: Vec<&str> = line.split_whitespace().collect();
            report
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let reports = parse_input(test_mode, 1);

    reports
        .iter()
        .filter(|report| check_is_safe(report))
        .count()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let reports = parse_input(test_mode, 1);

    reports
        .iter()
        .filter(|report| {
            if check_is_safe(report) {
                true
            } else {
                report.iter().enumerate().any(|(index, _)| {
                    let mut clone_report = report.to_vec();
                    clone_report.remove(index);
                    check_is_safe(&clone_report)
                })
            }
        })
        .count()
}

fn check_is_safe(report: &[i32]) -> bool {
    let is_increasing = report[0] < report[1];

    for window in report.windows(2) {
        let diff = window[0] - window[1];

        if !(1..=3).contains(&diff.abs()) {
            return false;
        }

        if is_increasing && diff.is_positive() || !is_increasing && diff.is_negative() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::day_2::{check_is_safe, solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 2);
    }
    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 4);
    }

    #[test]
    fn check_is_safe_should_be_safe() {
        let result = check_is_safe(&vec![7, 6, 4, 2, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn check_is_safe_should_not_be_safe() {
        let result = check_is_safe(&vec![9, 7, 6, 2, 1]);
        assert_eq!(result, false);
    }
}
