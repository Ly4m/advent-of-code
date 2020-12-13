use std::fs;
use std::ops::Mul;

pub fn solve_part_1() -> usize {
    let report: Vec<usize> = parse_input();
    repair_report(&report, 2020)
}

pub fn solve_part_2() -> usize {
    let report: Vec<usize> = parse_input();
    repair_report_2(&report, 2020)
}

fn parse_input() -> Vec<usize> {
    fs::read_to_string("inputs/day_1.in")
        .unwrap()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn repair_report(report: &Vec<usize>, year: usize) -> usize {
    for (index, value) in report.iter().enumerate() {
        for value2 in report[index..report.len()].iter() {
            if value + value2 == year {
                return value.mul(value2);
            }
        }
    }
    return 0;
}

fn repair_report_2(report: &Vec<usize>, year: usize) -> usize {
    for (index, value) in report.iter().enumerate() {
        for (index2, value2) in report[index..report.len()].iter().enumerate() {
            // no need to iterate a third time if
            if value + value2 < year {
                for value3 in report[index2..report.len()].iter() {
                    if value + value2 + value3 == year {
                        return value.mul(value2).mul(value3);
                    }
                }
            }
        }
    }
    return 0;  // should not happen
}

#[cfg(test)]
mod tests {
    use crate::day_1::{repair_report, repair_report_2};

    #[test]
    fn simple_repair_report() {
        let report: Vec<usize> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, repair_report(&report, 2020))
    }

    #[test]
    fn simple_repair_report_2() {
        let report: Vec<usize> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, repair_report_2(&report, 2020))
    }
}
