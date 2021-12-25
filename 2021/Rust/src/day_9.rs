use std::fs;

fn parse_input(test_mode: bool) -> Vec<Vec<u8>> {
    let path = if test_mode { "test_inputs/day_9.in" } else { "inputs/day_9.in" };
    fs::read_to_string(path).unwrap()
        .lines()
        .map(|line|
            line.split("")
                .filter(|x| !x.is_empty())
                .map(|n| n.parse().expect("Should be a number"))
                .collect::<Vec<u8>>()
        )
        .collect()
}


pub fn solve_part_1(test_mode: bool) -> usize {
    let toto = parse_input(test_mode);
    println!("{:?}", toto);
    0
}

pub fn solve_part_2(_test_mode: bool) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_9::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(15, result)
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(61229, result)
    }
}
