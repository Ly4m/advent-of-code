use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input<T: std::str::FromStr>(test_mode: bool, part: i8) -> Vec<(T, T)> {
    let path = if test_mode {
        format!("inputs_test/day_1_{}.in", part)
    } else {
        "inputs/day_1.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let mut locations = line.split_whitespace();
            match (
                locations.next()?.parse::<T>(),
                locations.next()?.parse::<T>(),
            ) {
                (Ok(loc1), Ok(loc2)) => Some((loc1, loc2)),
                _ => None,
            }
        })
        .collect::<Vec<(T, T)>>()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines = parse_input::<isize>(test_mode, 1);

    let (mut list_a, mut list_b): (Vec<_>, Vec<_>) = lines.into_iter().unzip();

    list_a.sort_unstable();
    list_b.sort_unstable();

    list_a
        .into_iter()
        .zip(list_b)
        .map(|(a, b)| (a - b).unsigned_abs())
        .sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let lines = parse_input::<usize>(test_mode, 1);

    let (list_a, list_b): (Vec<_>, Vec<_>) = lines.into_iter().unzip();

    let mut counts = HashMap::new();
    for &number in &list_b {
        *counts.entry(number).or_insert(0) += 1;
    }

    list_a
        .iter()
        .map(|&number| number * counts.get(&number).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_1::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 11);
    }
    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 31);
    }
}
