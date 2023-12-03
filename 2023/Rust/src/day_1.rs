use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item = String> {
    let path = if test_mode {
        format!("inputs_test/day_1_{}.in", part)
    } else {
        "inputs/day_1.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map_while(Result::ok)
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines = parse_input(test_mode, 1);
    let mut sum = 0;

    for line in lines {
        sum += find_first_and_last_digit_sum(line);
    }

    sum
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let lines = parse_input(test_mode, 2);
    let mut sum = 0;

    for line in lines {
        let interpolated_line = interpolate_numbers(line);
        sum += find_first_and_last_digit_sum(interpolated_line);
    }

    sum
}

fn find_first_and_last_digit_sum(line: String) -> usize {
    let mut num_1: char = char::default();
    let mut num_2: char = char::default();

    for c in line.chars() {
        if c.is_ascii_digit() {
            num_1 = c;
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            num_2 = c;
            break;
        }
    }

    format!("{num_1}{num_2}").parse::<usize>().unwrap()
}

pub fn interpolate_numbers(mut line: String) -> String {
    line = line.replace("one", "o1e");
    line = line.replace("two", "t2o");
    line = line.replace("three", "t3e");
    line = line.replace("four", "f4r");
    line = line.replace("five", "f5e");
    line = line.replace("six", "s6x");
    line = line.replace("seven", "s7n");
    line = line.replace("eight", "e8t");
    line = line.replace("nine", "n9e");
    line
}

#[cfg(test)]
mod tests {
    use crate::day_1::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 281);
    }
}
