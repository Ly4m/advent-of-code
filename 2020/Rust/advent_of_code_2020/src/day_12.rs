use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
#[derive(Debug)]
struct Instruction {
    action: char,
    value: isize,
}

fn parse_input() -> Vec<Instruction> {
    fs::read_to_string("inputs/day_12.in").expect("Missing input file")
        .lines()
        .map(String::from)
        .map(|line| -> Instruction {
            Instruction {
                action: line.chars().next().expect("missing char"),
                value: line.split_at(1).1.parse().expect("Not a number"),
            }
        })
        .collect()
}

pub fn solve_part_1() -> usize {
    let instructions = parse_input();
    let mut current_direction = 'E';
    let mut position: (isize, isize) = (0, 0);

    let directions: HashMap<char, isize> = [('E', 0), ('S', 90), ('W', 180), ('N', 270)].iter().cloned().collect();

    for instruction in instructions {
        position = if instruction.action == 'F' {
            navigate(&position, current_direction, instruction.value)
        } else if instruction.action == 'R' || instruction.action == 'L' {
            current_direction = turn(&current_direction, instruction.action, instruction.value, &directions);
            position
        } else {
            navigate(&position, instruction.action, instruction.value)
        }
    }

    (position.0.abs() + position.1.abs()) as usize
}

fn turn(current_direction: &char, orientation: char, value: isize, directions: &HashMap<char, isize>) -> char {
    let mut current_as_degree = *directions.get(current_direction).expect("Unknwon direction");
    let diff = if orientation == 'R' { value } else { 360 - value };

    current_as_degree = (current_as_degree + diff) % 360;

    directions.clone().into_iter()
        .filter(|(_key, value)| value == &current_as_degree)
        .map(|x| x.0).next()
        .expect("impossible")
}

fn navigate(position: &(isize, isize), direction: char, distance: isize) -> (isize, isize) {
    let mut new_position = position.clone();

    match direction {
        'N' => new_position.1 += distance,
        'S' => new_position.1 -= distance,
        'E' => new_position.0 += distance,
        'W' => new_position.0 -= distance,
        _ => panic!("Unknown instruction")
    }

    new_position
}

pub fn solve_part_2() -> usize {
    0
}
