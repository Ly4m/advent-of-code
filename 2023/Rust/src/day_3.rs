use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item = String> {
    let path = if test_mode {
        format!("inputs_test/day_3_{}.in", part)
    } else {
        "inputs/day_3.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

#[derive(Debug, Eq)]
struct Position {
    x: i32,
    y: i32,
    symbol: char,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Eq)]
struct Gear {
    x: i32,
    y: i32,
    parts: Vec<i32>,
}

impl PartialEq for Gear {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Part {
    value: i32,
    line: i32,
    start_col: i32,
    end_col: i32,
    accounted_for: bool,
}

fn find_symbols(test_mode: bool) -> Vec<Position> {
    let lines = parse_input(test_mode, 1);
    let mut symbols_coordinates: Vec<Position> = vec![];

    for (x, line) in lines.enumerate() {
        for (y, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                symbols_coordinates.push(Position {
                    x: x as i32,
                    y: y as i32,
                    symbol: char,
                });
            }
        }
    }

    symbols_coordinates
}

fn find_parts(test_mode: bool) -> Vec<Part> {
    let mut parts: Vec<Part> = vec![];
    let lines = parse_input(test_mode, 1);

    for (x, line) in lines.enumerate() {
        let mut current_part = String::new();

        for (y, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                current_part.push(char);
            } else if (y == line.len() - 1 && !current_part.is_empty())
                || (!char.is_ascii_digit() && !current_part.is_empty())
            {
                parts.push(Part {
                    value: current_part.to_string().parse::<i32>().unwrap(),
                    line: x as i32,
                    start_col: (y - current_part.len()) as i32,
                    end_col: (y - 1) as i32,
                    accounted_for: false,
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
            for y in part.start_col - 1..=part.end_col + 1 {
                if symbols_coordinates.contains(&Position { x, y, symbol: '$' }) {
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
    let mut gears: Vec<Gear> = symbols_coordinates
        .iter()
        .filter(|s| s.symbol == '*')
        .map(|s| Gear {
            parts: vec![],
            x: s.x,
            y: s.y,
        })
        .collect();

    for part in parts {
        for x in part.line - 1..=part.line + 1 {
            for y in part.start_col - 1..=part.end_col + 1 {
                if let Some(gear) = gears.iter_mut().find(|g| g.x == x && g.y == y) {
                    if !part.accounted_for {
                        gear.parts.push(part.value);
                    }
                }
            }
        }
    }

    gears
        .iter()
        .filter(|x| x.parts.len() == 2)
        .map(|x| x.parts.iter().product::<i32>())
        .sum::<i32>() as usize
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
