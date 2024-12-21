use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn parse_input(test_mode: bool, part: i8) -> Vec<Robot> {
    let path = if test_mode {
        format!("inputs_test/day_14_{}.in", part)
    } else {
        "inputs/day_14.in".to_owned()
    };

    let content = fs::read_to_string(path).unwrap();

    content.lines().map(parse_robot).collect()
}

fn parse_robot(line: &str) -> Robot {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let position_part = parts[0].trim_start_matches("p=").split_once(",").unwrap();
    let velocity_part = parts[1].trim_start_matches("v=").split_once(",").unwrap();

    let position: (isize, isize) = (
        position_part.0.trim().parse().unwrap(),
        position_part.1.trim().parse().unwrap(),
    );
    let velocity: (isize, isize) = (
        velocity_part.0.trim().parse().unwrap(),
        velocity_part.1.trim().parse().unwrap(),
    );

    Robot { position, velocity }
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let robots = parse_input(test_mode, 1);

    let width = if test_mode { 11 } else { 101 };
    let height = if test_mode { 7 } else { 103 };

    let mut quadrant_1 = 0;
    let mut quadrant_2 = 0;
    let mut quadrant_3 = 0;
    let mut quadrant_4 = 0;

    let quadrant_height = (height - 1) / 2;
    let quadrant_width = (width - 1) / 2;

    robots.iter().for_each(|robot| {
        let mut next_x = (robot.position.0 + (robot.velocity.0 * 100)) % width;
        let mut next_y = (robot.position.1 + (robot.velocity.1 * 100)) % height;

        if next_x < 0 {
            next_x += width
        }

        if next_y < 0 {
            next_y += height
        }

        if next_x < quadrant_width && next_y < quadrant_height {
            quadrant_1 += 1;
        }
        if next_x < quadrant_width && next_y > quadrant_height {
            quadrant_2 += 1;
        }
        if next_x > quadrant_width && next_y < quadrant_height {
            quadrant_3 += 1;
        }
        if next_x > quadrant_width && next_y > quadrant_height {
            quadrant_4 += 1;
        }
    });

    quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4
}

pub fn solve_part_2(test_mode: bool) -> usize {

    let robots = parse_input(test_mode, 1);

    let width = if test_mode { 11 } else { 101 };
    let height = if test_mode { 7 } else { 103 };

    for i in 1000..(width * height) {
        let mut grid = vec![vec![' '; width as usize]; height as usize];

        robots.iter().for_each(|robot| {
            let mut next_x = (robot.position.0 + (robot.velocity.0 * i)) % width;
            let mut next_y = (robot.position.1 + (robot.velocity.1 * i)) % height;

            if next_x < 0 {
                next_x += width
            }

            if next_y < 0 {
                next_y += height
            }

            grid[next_y as usize][next_x as usize] = '=';
        });
        for row in &grid {
            let r = row.iter().collect::<String>();
            if r.contains("=============================") {
                return i as usize;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day_14::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 12);
    }
}
