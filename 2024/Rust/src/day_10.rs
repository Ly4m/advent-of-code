use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> Vec<Vec<usize>> {
    let path = if test_mode {
        format!("inputs_test/day_10_{}.in", part)
    } else {
        "inputs/day_10.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn calculate_trail_score(
    (l, r): (isize, isize),
    map: &Vec<Vec<usize>>,
) -> Vec<(usize, usize)> {
    let current_value = map[l as usize][r as usize];

    if current_value == 9 {
        return vec![(l as usize, r as usize)];
    }

    let height = map.len();
    let width = map[0].len();
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut trails = Vec::new();

    for (dr, dl) in directions {
        if dr + r < width as isize
            && dr + r >= 0
            && dl + l < height as isize
            && dl + l >= 0
            && map[(dl + l) as usize][(dr + r) as usize] == (current_value + 1)
        {
            trails.append(&mut calculate_trail_score((dl + l, dr + r), map));
        }
    }

    trails
}

pub fn calculate_trail_score_distinct((l, r): (isize, isize), map: &Vec<Vec<usize>>) -> usize {
    let current_value = map[l as usize][r as usize];

    if current_value == 9 {
        return 1;
    }

    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut trail_scores = 0;

    for (dr, dl) in directions {
        let next_l = l + dl;
        let next_r = r + dr;
        if direction_in_grid(next_l, next_r, map, current_value)
        {
            trail_scores += calculate_trail_score_distinct((next_l, next_r), map);
        }
    }

    trail_scores
}

fn direction_in_grid(next_l: isize, next_r: isize, map: &[Vec<usize>], current_value: usize) -> bool {
    let height = map.len();
    let width = map[0].len();
    
    next_r< width as isize
        && next_r >= 0
        && next_l < height as isize
        && next_l >= 0
        && map[next_l as usize][next_r as usize] == (current_value + 1)
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let input = parse_input(test_mode, 1);
    let mut trails: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    for (l, line) in input.iter().enumerate() {
        for (r, row) in line.iter().enumerate() {
            if *row == 0 {
                let reaches = calculate_trail_score((l as isize, r as isize), &input);
                for x in reaches {
                    trails.insert(((l, r), x));
                }
            }
        }
    }

    trails.len()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let input = parse_input(test_mode, 1);

    let mut scores = 0;

    for (l, line) in input.iter().enumerate() {
        for (r, row) in line.iter().enumerate() {
            if *row == 0 {
                scores += calculate_trail_score_distinct((l as isize, r as isize), &input);
            }
        }
    }

    scores
}

#[cfg(test)]
mod tests {
    use crate::day_10::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 36);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 81);
    }
}
