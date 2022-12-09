use std::fs;

fn parse_input(test_mode: bool) -> Vec<(char, char)> {
    let path = if test_mode { "inputs_test/day_2.in" } else { "inputs/day_2.in" };
    let lines: Vec<(char, char)> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|round| {
            let mut split_line = round.split_whitespace();
            (split_line.next().unwrap(), split_line.next().unwrap())
        })
        .map(|x| (x.0.parse::<char>().unwrap(), x.1.parse::<char>().unwrap()))
        .collect();

    lines
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let tournament: Vec<(char, char)> = parse_input(test_mode);
    let sum: i32 = tournament.iter().map(|round| compute_score(round)).collect::<Vec<i32>>().iter().sum();
    sum as usize
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let mut tournament: Vec<(char, char)> = parse_input(test_mode);
    tournament = tournament.iter().map(| round: &(char, char)| (round.0, choose_shape(round))).collect();
    let score:i32 = tournament.iter().map(|round| compute_score(round)).collect::<Vec<i32>>().iter().sum();
    score as usize
}

pub fn choose_shape(round: &(char, char)) -> char {

    match round {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!(),
    }
}

pub fn compute_score(round: &(char, char)) -> i32 {
    let shape_score = match round.1 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unexpected param"),
    };

    let round_score = match round {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        _ => panic!(),
    };

    shape_score + round_score
}

#[cfg(test)]
mod tests {
    use crate::day_2::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 15);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 12);
    }
}
