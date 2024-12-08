use io::BufReader;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> Vec<Vec<char>> {
    let path = if test_mode {
        format!("inputs_test/day_6_{}.in", part)
    } else {
        "inputs/day_6.in".to_owned()
    };

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

fn find_starting_position(grid: &[Vec<char>]) -> (isize, isize, char) {
    for (l, line) in grid.iter().enumerate() {
        for (r, char) in line.iter().enumerate() {
            if ['<', '>', '^', 'v'].contains(char) {
                return (l as isize, r as isize, *char);
            }
        }
    }
    panic!("Could not find start position");
}

fn find_next_direction(current_direction: (isize, isize, char)) -> (isize, isize, char) {
    match current_direction {
        (0, 1, '>') => (1, 0, 'v'),
        (1, 0, 'v') => (0, -1, '<'),
        (0, -1, '<') => (-1, -0, '^'),
        (-1, 0, '^') => (0, 1, '>'),
        _ => panic!("Should not be happening"),
    }
}

fn is_grid_infinite(grid: &[Vec<char>]) -> bool {
    let mut all_visited_positions = vec![];

    let mut position = find_starting_position(grid);
    let mut direction = find_starting_direction(grid, &position);

    while guard_is_in_grid(grid, &position) {
        let mut new_position = (
            position.0 + direction.0,
            position.1 + direction.1,
            position.2,
        );

        if guard_is_leaving(grid, new_position) {
            break;
        }

        if new_position_is_obstacle(grid, &new_position) {
            direction = find_next_direction(direction);
            new_position = (
                position.0 + direction.0,
                position.1 + direction.1,
                direction.2,
            );
        }
        position = new_position;

        if all_visited_positions.contains(&position) {
            return true;
        }

        all_visited_positions.push(position);
    }
    false
}

fn new_position_is_obstacle(grid: &[Vec<char>], new_position: &(isize, isize, char)) -> bool {
    grid[new_position.0 as usize][new_position.1 as usize] == '#'
}

fn find_starting_direction(
    grid: &[Vec<char>],
    position: &(isize, isize, char),
) -> (isize, isize, char) {
    match grid[position.0 as usize][position.1 as usize] {
        '>' => (0, 1, '>'),
        'v' => (1, 0, 'v'),
        '<' => (0, -1, '<'),
        '^' => (-1, 0, '^'),
        _ => {
            panic!("Should not be happening")
        }
    }
}

fn guard_is_leaving(grid: &[Vec<char>], new_position: (isize, isize, char)) -> bool {
    new_position.0 < 0
        || new_position.1 < 0
        || new_position.0 >= (grid.len() as isize)
        || new_position.1 >= grid[0].len() as isize
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let grid = parse_input(test_mode, 1);
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();

    let mut position = find_starting_position(&grid);
    let mut direction = find_starting_direction(&grid, &position);

    while guard_is_in_grid(&grid, &position) {
        let mut new_position = (
            position.0 + direction.0,
            position.1 + direction.1,
            position.2,
        );

        if guard_is_leaving(&grid, new_position) {
            break;
        }

        while new_position_is_obstacle(&grid, &new_position) {
            direction = find_next_direction(direction);
            new_position = (
                position.0 + direction.0,
                position.1 + direction.1,
                direction.2,
            );
        }
        position = new_position;
        visited_positions.insert((position.0, position.1));
    }
    visited_positions.len()
}

pub fn solve_part_2(test_mode: bool) -> usize {

    let grid = parse_input(test_mode, 1);
    let mut position = find_starting_position(&grid);
    let starting = find_starting_position(&grid);
    let mut direction = find_starting_direction(&grid, &position);

    let mut visited_positions: HashSet<(isize, isize, char)> = HashSet::new();
    let mut infinite_loop_positions: HashSet<(isize, isize)> = HashSet::new();

    while guard_is_in_grid(&grid, &position) {
        let mut new_position = (
            position.0 + direction.0,
            position.1 + direction.1,
            position.2,
        );

        if guard_is_leaving(&grid, new_position) {
            break;
        }

        if new_position_is_obstacle(&grid, &new_position) {
            direction = find_next_direction(direction);
            new_position = (
                position.0 + direction.0,
                position.1 + direction.1,
                direction.2,
            );
        }

        if !(guard_is_leaving(&grid, new_position) || new_position.0 == starting.0 && new_position.1 == starting.1)
        {
            println!("{:?}", new_position);

            let mut clone = grid.clone();
            clone[new_position.0 as usize][new_position.1 as usize] = '#';
            if is_grid_infinite(&clone) {
                infinite_loop_positions.insert((new_position.0, new_position.1));
            }
        }

        position = new_position;
        visited_positions.insert(position);
    }
    infinite_loop_positions.len()
}

fn guard_is_in_grid(grid: &[Vec<char>], position: &(isize, isize, char)) -> bool {
    position.0 < (grid.len() as isize)
        && position.1 < (grid[position.0 as usize].len() as isize)
        && (position.0 >= 0)
        && (position.1 >= 0)
}

#[cfg(test)]
mod tests {
    use crate::day_6::{is_grid_infinite, solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 41);
    }
    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 6);
    }

    #[test]
    fn is_grid_infinite_should_not_be_infinite() {
        let grid: Vec<Vec<char>> = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "
        .lines()
        .map(|line| line.chars().collect())
        .collect();
        let result = is_grid_infinite(&grid);
        assert_eq!(result, false);
    }

    #[test]
    fn is_grid_infinite_should_be_infinite() {
        let grid: Vec<Vec<char>> = "....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#...
        "
        .lines()
        .map(|line| line.chars().collect())
        .collect();
        let result = is_grid_infinite(&grid);
        assert_eq!(result, true);
    }
}
