use std::fs;

#[derive(Debug, PartialEq)]
enum Tile {
    Robot,
    Floor,
    Wall,
    Box,
}

#[derive(Debug, PartialEq)]
enum WideTile {
    Robot,
    Floor,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Up,
    Down,
    Right,
    Left,
}

impl Instruction {
    fn new_position(&self, current_position: &(usize, usize)) -> (usize, usize) {
        let x = current_position.0;
        let y = current_position.1;
        match *self {
            Instruction::Left => (x, y - 1),
            Instruction::Right => (x, y + 1),
            Instruction::Up => (x - 1, y),
            Instruction::Down => (x + 1, y),
        }
    }
}

fn parse_input(test_mode: bool, part: i8) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let path = if test_mode {
        format!("inputs_test/day_15_{}.in", part)
    } else {
        "inputs/day_15.in".to_owned()
    };

    let content = fs::read_to_string(path).unwrap();
    let mut parts = content.split("\n\n");
    let warehouse = parse_warehouse(parts.next().unwrap());
    let instructions = parse_instructions(parts.next().unwrap());

    (warehouse, instructions)
}

fn parse_warehouse(block: &str) -> Vec<Vec<Tile>> {
    block
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    _ => panic!("Unknown tile: {}", char),
                })
                .collect()
        })
        .collect()
}

fn parse_instructions(block: &str) -> Vec<Instruction> {
    block
        .lines()
        .flat_map(|line| {
            line.chars().map(|char| match char {
                '^' => Instruction::Up,
                'v' => Instruction::Down,
                '>' => Instruction::Right,
                '<' => Instruction::Left,
                _ => panic!("Unknown instruction: {}", char),
            })
        })
        .collect()
}

fn can_move_boxes(
    warehouse: &[Vec<Tile>],
    instruction: &Instruction,
    box_position: &(usize, usize),
) -> bool {
    let next_position = instruction.new_position(box_position);
    let tile_at_next_position = &warehouse[next_position.0][next_position.1];

    match tile_at_next_position {
        Tile::Floor => true,
        Tile::Box => can_move_boxes(warehouse, instruction, &next_position),
        Tile::Wall => false,
        _ => panic!("Unknown tile: {:?}", tile_at_next_position),
    }
}

fn can_move_wide_boxes(
    warehouse: &[Vec<WideTile>],
    instruction: &Instruction,
    box_position: &(usize, usize),
) -> bool {
    let next_position = instruction.new_position(box_position);
    let tile_at_next_position = &warehouse[next_position.0][next_position.1];

    match tile_at_next_position {
        WideTile::Floor => true,
        WideTile::BoxLeft => {
            if instruction == &Instruction::Up || instruction == &Instruction::Down {
                let other_box_side = (next_position.0, next_position.1 + 1);
                can_move_wide_boxes(warehouse, instruction, &next_position)
                    && can_move_wide_boxes(warehouse, instruction, &other_box_side)
            } else {
                can_move_wide_boxes(warehouse, instruction, &next_position)
            }
        }
        WideTile::BoxRight => {
            if instruction == &Instruction::Up || instruction == &Instruction::Down {
                let other_box_side = (next_position.0, next_position.1 - 1);
                can_move_wide_boxes(warehouse, instruction, &next_position)
                    && can_move_wide_boxes(warehouse, instruction, &other_box_side)
            } else {
                can_move_wide_boxes(warehouse, instruction, &next_position)
            }
        }
        WideTile::Wall => false,
        _ => panic!("Unknown tile: {:?}", tile_at_next_position),
    }
}

fn move_boxes(
    warehouse: &mut [Vec<Tile>],
    instruction: &Instruction,
    box_position: &(usize, usize),
) {
    let next_position = instruction.new_position(box_position);
    let tile_at_next_position = &warehouse[next_position.0][next_position.1];

    match tile_at_next_position {
        Tile::Floor => {
            warehouse[next_position.0][next_position.1] = Tile::Box;
            warehouse[box_position.0][box_position.1] = Tile::Floor;
        }
        Tile::Box => {
            move_boxes(warehouse, instruction, &next_position);
            warehouse[next_position.0][next_position.1] = Tile::Box;
            warehouse[box_position.0][box_position.1] = Tile::Floor;
        }
        _ => panic!("Unknown tile: {:?}", tile_at_next_position),
    }
}

fn move_wide_boxes(
    warehouse: &mut [Vec<WideTile>],
    instruction: &Instruction,
    box_position: &(usize, usize),
) {
    let next_position = instruction.new_position(box_position);
    let tile_at_next_position = &warehouse[next_position.0][next_position.1];
    let is_left = warehouse[box_position.0][box_position.1] == WideTile::BoxLeft;

    match tile_at_next_position {
        WideTile::Floor => {
            warehouse[next_position.0][next_position.1] = if is_left {
                WideTile::BoxLeft
            } else {
                WideTile::BoxRight
            };
            warehouse[box_position.0][box_position.1] = WideTile::Floor;
        }
        WideTile::BoxLeft => {
            if instruction == &Instruction::Up || instruction == &Instruction::Down {
                let other_box_side = (next_position.0, next_position.1 + 1);
                move_wide_boxes(warehouse, instruction, &other_box_side);
            }
            move_wide_boxes(warehouse, instruction, &next_position);
            warehouse[next_position.0][next_position.1] = if is_left {
                WideTile::BoxLeft
            } else {
                WideTile::BoxRight
            };
            warehouse[box_position.0][box_position.1] = WideTile::Floor;
        }
        WideTile::BoxRight => {
            if instruction == &Instruction::Up || instruction == &Instruction::Down {
                let other_box_side = (next_position.0, next_position.1 - 1);
                move_wide_boxes(warehouse, instruction, &other_box_side);
            }
            move_wide_boxes(warehouse, instruction, &next_position);
            warehouse[next_position.0][next_position.1] = if is_left {
                WideTile::BoxLeft
            } else {
                WideTile::BoxRight
            };
            warehouse[box_position.0][box_position.1] = WideTile::Floor;
        }
        _ => panic!(
            "Unknown tile: {:?} at {:?}",
            tile_at_next_position, next_position
        ),
    }
}

fn move_in_direction_wide(
    warehouse: &mut [Vec<WideTile>],
    robot_position: (usize, usize),
    instruction: &Instruction,
) -> (usize, usize) {
    let next_position = instruction.new_position(&robot_position);
    let tile_at_next_position = &warehouse[next_position.0][next_position.1];

    match tile_at_next_position {
        WideTile::Floor => {
            warehouse[robot_position.0][robot_position.1] = WideTile::Floor;
            warehouse[next_position.0][next_position.1] = WideTile::Robot;
            next_position
        }
        WideTile::BoxLeft => {
            if instruction == &Instruction::Up || instruction == &Instruction::Down {
                let other_box_side = (next_position.0, next_position.1 + 1);

                if can_move_wide_boxes(warehouse, instruction, &next_position)
                    && can_move_wide_boxes(warehouse, instruction, &other_box_side)
                {
                    move_wide_boxes(warehouse, instruction, &other_box_side);
                    move_wide_boxes(warehouse, instruction, &next_position);
                    warehouse[next_position.0][next_position.1] = WideTile::Robot;
                    warehouse[robot_position.0][robot_position.1] = WideTile::Floor;
                    next_position
                } else {
                    robot_position
                }
            } else if can_move_wide_boxes(warehouse, instruction, &next_position) {
                move_wide_boxes(warehouse, instruction, &next_position);
                warehouse[next_position.0][next_position.1] = WideTile::Robot;
                warehouse[robot_position.0][robot_position.1] = WideTile::Floor;
                next_position
            } else {
                robot_position
            }
        }

        WideTile::BoxRight => {
            if instruction == &Instruction::Up || instruction == &Instruction::Down {
                let other_box_side = (next_position.0, next_position.1 - 1);

                if can_move_wide_boxes(warehouse, instruction, &next_position)
                    && can_move_wide_boxes(warehouse, instruction, &other_box_side)
                {
                    move_wide_boxes(warehouse, instruction, &other_box_side);
                    move_wide_boxes(warehouse, instruction, &next_position);
                    warehouse[next_position.0][next_position.1] = WideTile::Robot;
                    warehouse[robot_position.0][robot_position.1] = WideTile::Floor;
                    next_position
                } else {
                    robot_position
                }
            } else if can_move_wide_boxes(warehouse, instruction, &next_position) {
                move_wide_boxes(warehouse, instruction, &next_position);
                warehouse[next_position.0][next_position.1] = WideTile::Robot;
                warehouse[robot_position.0][robot_position.1] = WideTile::Floor;
                next_position
            } else {
                robot_position
            }
        }
        WideTile::Wall => robot_position,
        _ => panic!("Unknown tile: {:?}", tile_at_next_position),
    }
}

fn move_in_direction(
    warehouse: &mut [Vec<Tile>],
    robot_position: (usize, usize),
    instruction: &Instruction,
) -> (usize, usize) {
    let next_position = instruction.new_position(&robot_position);
    let tile_at_next_position = &warehouse[next_position.0][next_position.1];

    match tile_at_next_position {
        Tile::Floor => {
            warehouse[robot_position.0][robot_position.1] = Tile::Floor;
            warehouse[next_position.0][next_position.1] = Tile::Robot;
            next_position
        }
        Tile::Box => {
            if can_move_boxes(warehouse, instruction, &next_position) {
                move_boxes(warehouse, instruction, &next_position);
                warehouse[next_position.0][next_position.1] = Tile::Robot;
                warehouse[robot_position.0][robot_position.1] = Tile::Floor;
                next_position
            } else {
                robot_position
            }
        }
        Tile::Wall => robot_position,
        _ => panic!("Unknown tile: {:?}", tile_at_next_position),
    }
}

fn compute_gps(warehouse: &[Vec<Tile>]) -> usize {
    let mut count = 0;

    for (r, row) in warehouse.iter().enumerate() {
        for (t, _tile) in row.iter().enumerate() {
            if warehouse[r][t] == Tile::Box {
                count += 100 * r + t;
            }
        }
    }

    count
}

fn compute_gps_wide(warehouse: &[Vec<WideTile>]) -> usize {
    let mut count = 0;

    for (r, row) in warehouse.iter().enumerate() {
        for (t, _tile) in row.iter().enumerate() {
            if warehouse[r][t] == WideTile::BoxLeft {
                count += 100 * r + t;
            }
        }
    }

    count
}

fn widen_warehouse(warehouse: &[Vec<Tile>]) -> Vec<Vec<WideTile>> {
    let mut wide_warehouse = vec![];

    for row in warehouse.iter() {
        let mut new_row = vec![];
        for tile in row.iter() {
            match tile {
                Tile::Wall => {
                    new_row.push(WideTile::Wall);
                    new_row.push(WideTile::Wall);
                }
                Tile::Floor => {
                    new_row.push(WideTile::Floor);
                    new_row.push(WideTile::Floor);
                }
                Tile::Robot => {
                    new_row.push(WideTile::Robot);
                    new_row.push(WideTile::Floor);
                }
                Tile::Box => {
                    new_row.push(WideTile::BoxLeft);
                    new_row.push(WideTile::BoxRight);
                }
            }
        }
        wide_warehouse.push(new_row);
    }

    wide_warehouse
}

fn find_robot_starting_position(warehouse: &[Vec<Tile>]) -> (usize, usize) {
    for (x, row) in warehouse.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile == &Tile::Robot {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn find_robot_starting_position_wide(warehouse: &[Vec<WideTile>]) -> (usize, usize) {
    for (x, row) in warehouse.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile == &WideTile::Robot {
                return (x, y);
            }
        }
    }

    (0, 0)
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let (mut warehouse, instructions) = parse_input(test_mode, 1);
    let mut robot_position = find_robot_starting_position(&warehouse);

    for instruction in instructions {
        robot_position = move_in_direction(&mut warehouse, robot_position, &instruction);
    }

    compute_gps(&warehouse)
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let (warehouse, instructions) = parse_input(test_mode, 1);
    let mut wide_warehouse = widen_warehouse(&warehouse);
    let mut robot_position = find_robot_starting_position_wide(&wide_warehouse);

    for instruction in instructions {
        robot_position = move_in_direction_wide(&mut wide_warehouse, robot_position, &instruction);
    }

    compute_gps_wide(&wide_warehouse)
}

#[cfg(test)]
mod tests {
    use crate::day_15::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 10092);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 9021);
    }
}
