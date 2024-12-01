use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool) -> impl Iterator<Item=String> {
    const TEST_INPUT_PATH: &str = "inputs_test/";
    const INPUT_PATH: &str = "inputs/";

    let path = format!(
        "{}day_12.in",
        if test_mode {
            TEST_INPUT_PATH
        } else {
            INPUT_PATH
        }
    );
    let file = File::open(path).expect("File not found");
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

fn count_arrangements(pattern: &str, groups: &[i8] ) -> usize {
:
    0
}

fn map_to_spring(line: String) -> (String, Vec<i8>){
    let parts = line.split_once(' ').unwrap();
    (String::from(parts.0), parts.1.split(',').map(|x| x.parse::<i8>().unwrap()).collect())
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines: Vec<(String, Vec<i8>)> = parse_input(test_mode)
        .map(map_to_spring)
        .collect();

    println!("{:?}", lines);
    0
}

pub fn solve_part_2(_test_mode: bool) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_12::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 21);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 82000210);
    }
}
