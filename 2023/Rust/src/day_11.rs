use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::{Itertools};
use num::abs;

fn parse_input(test_mode: bool) -> impl Iterator<Item=String> {
    const TEST_INPUT_PATH: &str = "inputs_test/";
    const INPUT_PATH: &str = "inputs/";

    let path = format!(
        "{}day_11.in",
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

pub fn solve_part_1(test_mode: bool) -> usize {
    let mut universe: Vec<Vec<char>> = parse_input(test_mode)
        .map(|line| line.chars().collect())
        .collect();

    let mut new_universe = Vec::new();
    for row in &universe {
        new_universe.push(row.clone());
        if !row.contains(&'#') {
            new_universe.push(row.clone());
        }
    }
    universe = new_universe;

    let num_cols = universe[0].len();
    let mut cols_to_expand: Vec<usize> = vec![];

    for col in 0..num_cols {
        let mut duplicate_col = true;

        for row in &universe {
            if row[col] == '#' {
                duplicate_col = false;
                break;
            }
        }
        if duplicate_col {
            cols_to_expand.push(col);
        }
    }

    for (index, col) in cols_to_expand.iter().enumerate() {
        for row in &mut universe {
            row.insert(index + col, '.');
        }
    }

    let mut galaxies = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, galaxy) in row.iter().enumerate() {
            if galaxy == &'#' {
                galaxies.push((x, y))
            }
        }
    }

    let combinations: Vec<(&(usize, usize), &(usize, usize))> = galaxies.iter().tuple_combinations().collect();
    let mut sum = 0;

    for group in combinations {
        let distance = abs(group.1.0 as isize - group.0.0 as isize) + abs(group.1.1 as isize - group.0.1 as isize);
        sum += distance;
    }

    sum as usize
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let universe: Vec<Vec<char>> = parse_input(test_mode)
        .map(|line| line.chars().collect())
        .collect();

    let mut cols_to_expand: Vec<usize> = vec![];
    let mut rows_to_expand: Vec<usize> = vec![];

    for (index, row) in universe.iter().enumerate() {
        if !row.contains(&'#') {
            rows_to_expand.push(index);
        }
    }

    let num_cols = universe[0].len();
    for col in 0..num_cols {
        let mut duplicate_col = true;

        for row in &universe {
            if row[col] == '#' {
                duplicate_col = false;
                break;
            }
        }
        if duplicate_col {
            cols_to_expand.push(col);
        }
    }


    let mut galaxies = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, galaxy) in row.iter().enumerate() {
            if galaxy == &'#' {
                galaxies.push((x, y))
            }
        }
    }

    let scaled_galaxy: Vec<(usize, usize)> = galaxies.iter()
        .map(|g| scale_position(g, &rows_to_expand, &cols_to_expand))
        .collect();

    let combinations: Vec<(&(usize, usize), &(usize, usize))> = scaled_galaxy
        .iter()
        .tuple_combinations()
        .collect();
    let mut sum = 0;

    for group in combinations {
        let distance = manathan_distance(group.0, group.1);
        sum += distance;
    }

    sum as usize
}

fn scale_position(position: &(usize, usize), rows_to_expand: &[usize], cols_to_expand: &[usize]) -> (usize, usize) {
    let row_count = rows_to_expand.iter().take_while(|x| x < &&position.1).count();
    let col_count = cols_to_expand.iter().take_while(|y| y < &&position.0).count();
    (position.0 + (col_count * 999999), position.1 + (row_count * 999999))
}

fn manathan_distance(position_0: &(usize, usize), position_1: &(usize, usize)) -> isize {
    abs(position_1.0 as isize - position_0.0 as isize) + abs(position_1.1 as isize - position_0.1 as isize)
}

#[cfg(test)]
mod tests {
    use crate::day_11::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 374);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 82000210);
    }
}
