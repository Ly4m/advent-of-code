use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Div;

fn parse_input(test_mode: bool) -> impl Iterator<Item = String> {
    const TEST_INPUT_PATH: &str = "inputs_test/";
    const INPUT_PATH: &str = "inputs/";

    let path = format!(
        "{}day_10.in",
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

#[derive(Debug)]
struct PipePart {
    tile: char,
    is_in_loop: bool,
}

fn find_start(sketch: &Vec<Vec<PipePart>>) -> (usize, usize) {
    let mut index = None;

    for (i, row) in sketch.iter().enumerate() {
        if index.is_some() {
            break;
        }

        for (j, ch) in row.iter().enumerate() {
            if ch.tile == 'S' {
                index = Some((i, j));
            }
        }
    }

    index.unwrap()
}

fn look_up(sketch: &[Vec<PipePart>], origin: (usize, usize)) -> Option<(usize, usize)> {
    let up = (origin.0 - 1, origin.1);
    if vec!['|', '7', 'F', 'S'].contains(&sketch.get(up.0).unwrap().get(up.1).unwrap().tile) {
        return Some(up);
    }
    None
}

fn look_right(sketch: &[Vec<PipePart>], origin: (usize, usize)) -> Option<(usize, usize)> {
    let right = (origin.0, origin.1 + 1);
    if vec!['7', 'J', '-', 'S'].contains(&sketch.get(right.0).unwrap().get(right.1).unwrap().tile) {
        return Some(right);
    }
    None
}

fn look_down(sketch: &[Vec<PipePart>], origin: (usize, usize)) -> Option<(usize, usize)> {
    let down = (origin.0 + 1, origin.1);
    if vec!['|', 'J', 'L', 'S'].contains(&sketch.get(down.0).unwrap().get(down.1).unwrap().tile) {
        return Some(down);
    }
    None
}

fn look_left(sketch: &[Vec<PipePart>], origin: (usize, usize)) -> Option<(usize, usize)> {
    let left = (origin.0, origin.1 - 1);
    if vec!['-', 'L', 'F', 'S'].contains(&sketch.get(left.0).unwrap().get(left.1).unwrap().tile) {
        return Some(left);
    }
    None
}

fn find_next_position(
    sketch: &mut [Vec<PipePart>],
    current_position: (usize, usize),
    previous_position: (usize, usize),
) -> Option<(usize, usize)> {
    let origin_char = sketch
        .get(current_position.0)
        .unwrap()
        .get(current_position.1)
        .unwrap()
        .tile;
    match origin_char {
        '|' => {
            if previous_position.0 > current_position.0 {
                look_up(sketch, current_position)
            } else {
                look_down(sketch, current_position)
            }
        }
        '-' => {
            if previous_position.1 > current_position.1 {
                look_left(sketch, current_position)
            } else {
                look_right(sketch, current_position)
            }
        }
        'L' => {
            if previous_position.1 > current_position.1 {
                look_up(sketch, current_position)
            } else {
                look_right(sketch, current_position)
            }
        }
        'J' => {
            if previous_position.1 < current_position.1 {
                look_up(sketch, current_position)
            } else {
                look_left(sketch, current_position)
            }
        }
        '7' => {
            if previous_position.1 < current_position.1 {
                look_down(sketch, current_position)
            } else {
                look_left(sketch, current_position)
            }
        }
        'F' => {
            if previous_position.1 > current_position.1 {
                look_down(sketch, current_position)
            } else {
                look_right(sketch, current_position)
            }
        }
        'S' => look_right(sketch, current_position).or_else(|| {
            look_up(sketch, current_position).or_else(|| {
                look_left(sketch, current_position).or_else(|| look_down(sketch, current_position))
            })
        }),
        _ => None,
    }
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines = parse_input(test_mode);
    let mut sketch: Vec<Vec<PipePart>> = lines
        .map(|l| {
            l.chars()
                .map(|c| PipePart {
                    is_in_loop: false,
                    tile: c,
                })
                .collect()
        })
        .collect();
    let start_position = find_start(&sketch);
    let mut previous_position = start_position;
    let mut current_position = start_position;
    let mut steps: usize = 0;

    loop {
        let option = find_next_position(&mut sketch, current_position, previous_position);
        let next_position = option.expect("Should find next position");
        previous_position = current_position;
        current_position = next_position;
        steps += 1;
        if current_position.0 == start_position.0 && current_position.1 == start_position.1 {
            break;
        }
    }

    steps.div(2)
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let lines = parse_input(test_mode);
    let mut sketch: Vec<Vec<PipePart>> = lines
        .map(|l| {
            l.chars()
                .map(|c| PipePart {
                    is_in_loop: false,
                    tile: c,
                })
                .collect()
        })
        .collect();
    let start_position = find_start(&sketch);
    let mut previous_position = start_position;
    let mut current_position = start_position;

    loop {
        let option = find_next_position(&mut sketch, current_position, previous_position);
        let next_position = option.expect("Should find next position");
        previous_position = current_position;
        current_position = next_position;
        sketch
            .get_mut(current_position.0)
            .unwrap()
            .get_mut(current_position.1)
            .unwrap()
            .is_in_loop = true;
        if current_position.0 == start_position.0 && current_position.1 == start_position.1 {
            break;
        }
    }

    let mut points_in = 0;
    let mut is_in = false;
    let mut sequence_start: Option<char> = None;
    sketch
        .get_mut(start_position.0)
        .unwrap()
        .get_mut(start_position.1)
        .unwrap()
        .tile = '-';

    for (_x, line) in sketch.iter().enumerate() {
        for (_y, pipe) in line.iter().enumerate() {
            if is_in && !pipe.is_in_loop {
                points_in += 1;
            }
            if pipe.is_in_loop && ['|'].contains(&pipe.tile) {
                is_in = !is_in;
                sequence_start = None;
            }

            if pipe.tile == '.' {
                sequence_start = None;
            }

            if pipe.is_in_loop && ['F', 'J', '7', 'L'].contains(&pipe.tile) {
                match sequence_start {
                    Some('F') => {
                        if pipe.tile == 'J' {
                            is_in = !is_in;
                        }
                        sequence_start = None;
                    }
                    Some('L') => {
                        if pipe.tile == '7' {
                            is_in = !is_in;
                        }
                        sequence_start = None;
                    }
                    Some('J') | Some('7') => {
                        sequence_start = None;
                    }
                    None => {
                        sequence_start = Some(pipe.tile);
                    }
                    _ => {}
                }
            }
        }
        sequence_start = None;
        is_in = false;
    }
    points_in
}

#[cfg(test)]
mod tests {
    use crate::day_10::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 1);
    }
}
