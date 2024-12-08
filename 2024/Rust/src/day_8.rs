use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> Vec<Vec<char>> {
    let path = if test_mode {
        format!("inputs_test/day_8_{}.in", part)
    } else {
        "inputs/day_8.in".to_owned()
    };

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_antinodes(
    antennas: &[(isize, isize)],
    width: usize,
    height: usize,
) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::new();

    for ant in antennas.iter().combinations(2) {
        let a = ant[0];
        let b = ant[1];
        let c = (a.0 - b.0, a.1 - b.1);

        let d = (a.0 + c.0, a.1 + c.1);
        let e = (b.0 - c.0, b.1 - c.1);

        if (0..width as isize).contains(&d.0) && (0..height as isize).contains(&d.1) {
            antinodes.insert(d);
        }
        if (0..width as isize).contains(&e.0) && (0..height as isize).contains(&e.1) {
            antinodes.insert(e);
        }
    }

    antinodes
}

fn calculate_antinodes_with_harmonics(
    antennas: &[(isize, isize)],
    width: usize,
    height: usize,
) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::new();

    for antenna in antennas.iter().combinations(2) {
        let a = *antenna[0];
        let b = *antenna[1];
        let c = (a.0 - b.0, a.1 - b.1);

        let mut d = (a.0 + c.0, a.1 + c.1);
        let mut e = (b.0 - c.0, b.1 - c.1);

        antinodes.insert(a);
        antinodes.insert(b);

        while (0..width as isize).contains(&d.0) && (0..height as isize).contains(&d.1) {
            antinodes.insert(d);
            d = (d.0 + c.0, d.1 + c.1);
        }

        while (0..width as isize).contains(&e.0) && (0..height as isize).contains(&e.1) {
            antinodes.insert(e);
            e = (e.0 - c.0, e.1 - c.1);
        }
    }

    antinodes
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let grid = parse_input(test_mode, 1);
    let positions_by_antenna = group_antennas_positions(&grid);

    positions_by_antenna
        .values()
        .map(|positions| calculate_antinodes(positions, grid.len(), grid[0].len()))
        .reduce(|mut a, b| {
            a.extend(b);
            a
        })
        .map(|positions| positions.len())
        .unwrap()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let grid = parse_input(test_mode, 1);

    let positions_by_antenna = group_antennas_positions(&grid);

    positions_by_antenna
        .values()
        .map(|positions| calculate_antinodes_with_harmonics(positions, grid.len(), grid[0].len()))
        .reduce(|mut a, b| {
            a.extend(b);
            a
        })
        .map(|positions| positions.len())
        .unwrap()
}

fn group_antennas_positions(grid: &[Vec<char>]) -> HashMap<&char, Vec<(isize, isize)>> {
    let mut positions_by_antenna: HashMap<&char, Vec<(isize, isize)>> = HashMap::new();
    grid.iter().enumerate().for_each(|(l_idx, line)| {
        line.iter().enumerate().for_each(|(r_idx, char)| {
            if *char != '.' {
                let current_position = positions_by_antenna.get_mut(char);
                match current_position {
                    Some(position) => {
                        position.push((l_idx as isize, r_idx as isize));
                    }
                    None => {
                        positions_by_antenna.insert(char, vec![(l_idx as isize, r_idx as isize)]);
                    }
                }
            }
        })
    });
    positions_by_antenna
}

#[cfg(test)]
mod tests {
    use crate::day_8::{
        calculate_antinodes, calculate_antinodes_with_harmonics, solve_part_1, solve_part_2,
    };
    use std::collections::HashSet;
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 14);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 34);
    }

    #[test]
    fn part_1_calculate_antinodes() {
        let result = calculate_antinodes(&[(3, 4), (5, 5)], 10, 10);
        assert_eq!(result, HashSet::from([(1, 3), (7, 6)]));
    }

    #[test]
    fn part_1_calculate_antinodes_3_antennas() {
        let result = calculate_antinodes(&[(3, 4), (5, 5), (4, 8)], 10, 10);
        assert_eq!(result, HashSet::from([(1, 3), (7, 6), (2, 0), (6, 2)]));
    }

    #[test]
    fn part_2_calculate_antinodes_with_harmonics() {
        let result = calculate_antinodes_with_harmonics(&[(0, 0), (1, 3), (2, 1)], 10, 10);
        assert_eq!(
            result,
            HashSet::from([
                (0, 0),
                (1, 3),
                (2, 1),
                (0, 5),
                (2, 6),
                (3, 9),
                (4, 2),
                (6, 3),
                (8, 4)
            ])
        );
    }
}
