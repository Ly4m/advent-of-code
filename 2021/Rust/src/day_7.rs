use std::fs;

fn parse_input(test_mode: bool) -> Vec<usize> {
    let path = if test_mode { "test_inputs/day_7.in" } else { "inputs/day_7.in" };
    fs::read_to_string(path).unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().expect("Parsing issue"))
        .collect()
}


pub fn solve_part_1(test_mode: bool) -> usize {
    let crab_positions = parse_input(test_mode);
    let max = *crab_positions.iter().max().expect("Max not found");
    let min = *crab_positions.iter().min().expect("Min not found");

    (min..=max).into_iter()
        .map(|pos| calculate_fuel_for_position(&pos, &crab_positions))
        .min().expect("Min not found")
}

fn calculate_fuel_for_position(position: &usize, crab_positions: &[usize]) -> usize {
    crab_positions.iter().map(|pos| if pos < position { position - pos } else { pos - position }).sum()
}


pub fn solve_part_2(test_mode: bool) -> usize {
    let crab_positions = parse_input(test_mode);
    let max = *crab_positions.iter().max().expect("Max not found");
    let min = *crab_positions.iter().min().expect("Max not found");

    (min..=max).into_iter()
        .map(|pos| calculate_fuel_for_position_increasing(&pos, &crab_positions))
        .min().expect("Min not found")
}

fn calculate_fuel_for_position_increasing(position: &usize, crab_positions: &[usize]) -> usize {
    crab_positions.iter().map(|pos| {
        let distance = if pos < position { position - pos } else { pos - position };
        (distance * (distance + 1)) / 2
    }).sum()
}



#[cfg(test)]
mod tests {
    use crate::day_7::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(37, result)
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(168, result)
    }

}
