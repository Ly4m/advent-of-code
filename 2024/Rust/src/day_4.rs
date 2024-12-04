use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> Vec<Vec<char>> {
    let path = if test_mode {
        format!("inputs_test/day_4_{}.in", part)
    } else {
        "inputs/day_4.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let data = parse_input(test_mode, 1);
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut count = 0;
    for r in 0..data.len() {
        for c in 0..data[0].len() {
            for &(dr, dc) in &directions {
                if find_xmas_in_direction(&data, r as isize, c as isize, dr, dc) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_xmas_in_direction(
    grid: &[Vec<char>],
    start_row: isize,
    start_col: isize,
    row_direction: isize,
    col_direction: isize,
) -> bool {
    let target_chars: Vec<char> = vec!['X', 'M', 'A', 'S'];

    target_chars.iter().enumerate().all(|(i, &ch)| {
        let row = start_row + row_direction * i as isize;
        let col = start_col + col_direction * i as isize;
        grid.get(row as usize)
            .and_then(|r| r.get(col as usize))
            .map_or(false, |&cell| cell == ch)
    })
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let data = parse_input(test_mode, 1);
    let rows = data.len();
    let cols = data[0].len();
    let mut count = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if is_x_mas(&data, r as isize, c as isize) {
                count += 1;
            }
        }
    }
    count
}

fn is_x_mas(grid: &[Vec<char>], start_row: isize, start_col: isize) -> bool {
    let r = start_row;
    let c = start_col;

    if *grid[r as usize].get(c as usize).unwrap() != 'A' {
        false
    } else {
        ((*grid[(r - 1) as usize].get((c - 1) as usize).unwrap() == 'S'
            && *grid[(r + 1) as usize].get((c + 1) as usize).unwrap() == 'M')
            || (*grid[(r - 1) as usize].get((c - 1) as usize).unwrap() == 'M'
                && *grid[(r + 1) as usize].get((c + 1) as usize).unwrap() == 'S'))
            && ((*grid[(r + 1) as usize].get((c - 1) as usize).unwrap() == 'S'
                && *grid[(r - 1) as usize].get((c + 1) as usize).unwrap() == 'M')
                || (*grid[(r + 1) as usize].get((c - 1) as usize).unwrap() == 'M'
                    && *grid[(r - 1) as usize].get((c + 1) as usize).unwrap() == 'S'))
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 18);
    }
    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 9);
    }
}
