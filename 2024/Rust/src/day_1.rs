use std::fs::File;
use std::io;
use std::io::BufRead;
use num::abs;

fn parse_input(test_mode: bool, part: i8) -> impl Iterator<Item = String> {
    let path = if test_mode {
        format!("inputs_test/day_1_{}.in", part)
    } else {
        "inputs/day_1.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().map_while(Result::ok)
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines = parse_input(test_mode, 1);

    let mut list_a: Vec<isize> = vec![];
    let mut list_b: Vec<isize> = vec![];
    let mut sum: usize = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let (Ok(num1), Ok(num2)) = (parts[0].parse::<isize>(), parts[1].parse::<isize>()) {
            list_a.push(num1);
            list_b.push(num2);
        }
    }

    list_a.sort();
    list_b.sort();

    let it  = list_a.iter().zip(list_b.iter());

    for (i, (x, y)) in it.enumerate() {
        sum += abs(x - y) as usize;
    }

    sum
}

pub fn solve_part_2(test_mode: bool) -> usize {
    // let lines = parse_input(test_mode, 2);
    // let mut sum = 0;
    //
    // for line in lines {
    //     let interpolated_line = interpolate_numbers(line);
    //     sum += find_first_and_last_digit_sum(interpolated_line);
    // }
    //
    // sum
    0
}

#[cfg(test)]
mod tests {
    use crate::day_1::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 11);
    }

    // #[test]
    // fn part_2() {
    //     let result = solve_part_2(true);
    //     assert_eq!(result, 281);
    // }
}
