use std::fs;

fn parse_input(test_mode: bool) -> Vec<usize> {
    let path = if test_mode { "inputs_test/day_1.in" } else { "inputs/day_1.in" };
    let lines: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut calories_by_elf = Vec::new();
    let mut current_calories: usize = 0;

    for line in lines {
        match line.parse::<usize>() {
            Ok(calories) => current_calories += calories,
            Err(_) => {
                calories_by_elf.push(current_calories);
                current_calories = 0;
            }
        }
    }

    calories_by_elf.sort();
    calories_by_elf.reverse();
    calories_by_elf

}

pub fn solve_part_1(test_mode: bool) -> usize {
    let calories_by_elf = parse_input(test_mode);
    calories_by_elf[0]
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let calories_by_elf = parse_input(test_mode);
    calories_by_elf[0..=3].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_1::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 45000);
    }
}
