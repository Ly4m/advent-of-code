use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

const BASE_SCORE: usize = 2;
const ZERO_COUNT_SCORE: usize = 0;

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item = String> {
    let path = if test_mode {
        format!("inputs_test/day_4_{}.in", part)
    } else {
        "inputs/day_4.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

fn parse_line_to_game(line: &str) -> Game {
    let game_parts: Vec<&str> = line.split(':').collect();
    let id = game_parts[0]
        .replace("Card", "")
        .trim()
        .parse::<i32>()
        .unwrap();
    let numbers_part: Vec<&str> = game_parts[1].split('|').collect();

    let numbers: HashSet<i32> = numbers_part[0]
        .split_whitespace()
        .map(str::trim)
        .map(str::parse)
        .map(|x| x.unwrap())
        .collect();

    let winning_numbers = numbers_part[1]
        .split_whitespace()
        .map(str::trim)
        .map(str::parse)
        .map(|x| x.unwrap())
        .collect::<HashSet<i32>>();

    Game {
        id,
        numbers,
        winning_numbers,
    }
}

fn parse_lines_to_games(lines: impl Iterator<Item = String>) -> Vec<Game> {
    lines.map(|line| parse_line_to_game(&line)).collect()
}

fn calculate_points(game: &Game) -> usize {
    let count = game.numbers.intersection(&game.winning_numbers).count();
    match count {
        0 => ZERO_COUNT_SCORE,
        _ => BASE_SCORE.pow((count - 1) as u32),
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    numbers: HashSet<i32>,
    winning_numbers: HashSet<i32>,
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines = parse_input(test_mode, 1);
    let games = parse_lines_to_games(lines);

    games.iter().map(|g| calculate_points(g)).sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let lines = parse_input(test_mode, 1);
    let games = parse_lines_to_games(lines);
    let mut card_count: HashMap<i32, i32> = games.iter().map(|g| (g.id, 1)).collect();
    games.iter().for_each(|g| {
        let count: i32 = g.numbers.intersection(&g.winning_numbers).count() as i32;
        let end_range = if (g.id + count) > (games.len() + 1) as i32 {
            (games.len() + 1) as i32
        } else {
            g.id + count
        };

        for _card_copy in 0..*(card_count.get(&g.id).unwrap()) {
            for card in (g.id + 1)..=end_range {
                card_count.insert(card, card_count.get(&card).unwrap() + 1);
            }
        }
    });

    card_count.values().sum::<i32>() as usize
}

#[cfg(test)]
mod tests {
    use crate::day_4::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 30);
    }
}
