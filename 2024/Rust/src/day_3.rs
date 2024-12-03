use regex::Regex;
use std::fs::read_to_string;

fn parse_input(test_mode: bool, part: i8) -> String {
    let path = if test_mode {
        format!("inputs_test/day_3_{}.in", part)
    } else {
        "inputs/day_3.in".to_owned()
    };

    read_to_string(path).unwrap()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let data = parse_input(test_mode, 1);
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&data)
        .map(|x| {
            (
                x[1].parse::<usize>().unwrap(),
                x[2].parse::<usize>().unwrap(),
            )
        })
        .map(|(x, y)| x * y)
        .sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let data = parse_input(test_mode, 2);
    let re = Regex::new(r"(?:don't|do)\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut toggle = true;
    re.captures_iter(&data)
        .filter_map(|x| {
            if let Some(matched) = x.get(0) {
                match matched.as_str() {
                    "don't()" => {
                        toggle = false;
                        None
                    }
                    "do()" => {
                        toggle = true;
                        None
                    }
                    other if other.starts_with("mul") && toggle => 
                        Some(x[1].parse::<usize>().unwrap() * x[2].parse::<usize>().unwrap()
                    ),
                    &_ => None,
                }
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_3::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 161);
    }
    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 48);
    }
}
