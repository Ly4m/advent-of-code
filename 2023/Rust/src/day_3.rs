use std::io;
use std::fs::File;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item=String> {
    let path = if test_mode { format!("inputs_test/day_3_{}.in", part) } else { "inputs/day_3.in".to_owned() };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map_while(Result::ok)
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Part {
    value: i32,
    line: i32,
    start_col: i32,
    end_col: i32,
}

fn find_symbols(test_mode: bool) -> Vec<Position> {
    let lines = parse_input(test_mode, 1);
    let mut symbols_coordinates: Vec<Position> = vec![];

    for (x, line) in lines.enumerate() {
        for (y, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                symbols_coordinates.push(Position { x: x as i32, y: y as i32 });
            }
        }
    }

    symbols_coordinates
}


fn find_parts(test_mode: bool) -> Vec<Part> {
    let mut parts: Vec<Part> = vec![];
    let lines = parse_input(test_mode, 1);

    for (x, line) in lines.enumerate() {
        let mut current_part = String::from("");

        for (y, char) in line.chars().enumerate() {
            if y == line.len(){
                break;
            }
            // is number
            if char.is_ascii_digit() {
                current_part.push(char);
            }

            if (y == line.len() - 1 &&  current_part.len() > 0) || (!char.is_ascii_digit() && current_part.len() > 0) {
                parts.push(Part {
                    value: format!("{current_part}").parse::<i32>().unwrap(),
                    line: x as i32,
                    start_col: (y - current_part.len()) as i32,
                    end_col: (y - 1) as i32,
                });

                current_part.clear();
            }
        }
    }

    parts
}


pub fn solve_part_1(test_mode: bool) -> usize {
    let symbols_coordinates: Vec<Position> = find_symbols(test_mode);
    let parts: Vec<Part> = find_parts(test_mode);
    let mut sum = 0;

    for part in parts {
        for x in part.line - 1..=part.line + 1 {
            for y in part.start_col-1..=part.end_col + 1 {
                if symbols_coordinates.contains(&Position { x, y }) {
                    println!("FOUND : {:?}", part.value);
                    sum += part.value;
                };
            }
        }
    }


    sum as usize
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let symbols_coordinates: Vec<Position> = find_symbols(test_mode);
    let parts: Vec<Part> = find_parts(test_mode);

    0
}

#[cfg(test)]
mod tests {
    use crate::day_3::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 467835);
    }
}
