use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item = Game> {
    let path = if test_mode {
        format!("inputs_test/day_2_{}.in", part)
    } else {
        "inputs/day_2.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map_while(Result::ok).map(|x| parse_line(&x))
}

fn parse_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();
    let id_part = parts[0];
    let color_parts = parts[1].split("; ");

    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for part in color_parts {
        for handful in part.split(';') {
            for color_count in handful.split(", ") {
                let color_split: Vec<&str> = color_count.split_whitespace().collect();
                let count: i8 = color_split[0].parse().unwrap_or(0);

                match color_split[1] {
                    "red" => {
                        if red <= count {
                            red = count
                        }
                    }
                    "blue" => {
                        if blue <= count {
                            blue = count
                        }
                    }
                    "green" => {
                        if green <= count {
                            green = count
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    let id: i8 = id_part.replace("Game ", "").parse().unwrap_or(0);

    Game {
        id,
        red,
        blue,
        green,
    }
}

#[derive(Debug)]
struct Game {
    id: i8,
    red: i8,
    blue: i8,
    green: i8,
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let games = parse_input(test_mode, 1);
    let mut sum: i32 = 0;

    for game in games {
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            sum += i32::from(game.id);
        }
    }

    sum as usize
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let games = parse_input(test_mode, 2);
    let mut sum: i32 = 0;

    for game in games {
        sum += i32::from(game.red) * i32::from(game.green) * i32::from(game.blue);
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use crate::day_2::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 8);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 2286);
    }
}
