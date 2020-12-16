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
        position = match instruction.action {
            'F' => navigate(&position, current_direction, instruction.value),
            'R' | 'L' => {
                current_direction = turn(&current_direction, instruction.action, instruction.value, &directions);
                position
            }
            _ => navigate(&position, instruction.action, instruction.value)
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
    let mut new_position = *position;

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
    let instructions = parse_input();
    let mut waypoint_position: (isize, isize) = (10, 1);
    let mut ship_position: (isize, isize) = (0, 0);

    for instruction in instructions {
        match instruction.action {
            'F' => ship_position = move_ship_to_waypoint(ship_position, waypoint_position, instruction.value),
            'R' | 'L' => waypoint_position = rotate_waypoint(waypoint_position, instruction.action, instruction.value),
            _ => waypoint_position = navigate(&waypoint_position, instruction.action, instruction.value)
        }
    }

    (ship_position.0.abs() + ship_position.1.abs()) as usize
}

fn move_ship_to_waypoint(ship_position: (isize, isize), waypoint_position: (isize, isize), value: isize) -> (isize, isize) {
    (ship_position.0 + (waypoint_position.0 * value), ship_position.1 + (waypoint_position.1 * value))
}

fn rotate_waypoint(waypoint_position: (isize, isize), orientation: char, value: isize) -> (isize, isize) {
    let move_value = if orientation == 'R' { value } else { 360 - value };

    match move_value {
        90 => (waypoint_position.1, -waypoint_position.0),
        180 => (-waypoint_position.0, -waypoint_position.1),
        270 => (-waypoint_position.1, waypoint_position.0),
        _ => panic!("Not happening")
    }
}


#[cfg(test)]
mod tests {
    use crate::day_12::move_ship_to_waypoint;

    #[test]
    fn test_waypoint_forward() {
        assert_eq!((100, 10), move_ship_to_waypoint((0, 0), (10, 1), 10))
    }
}
