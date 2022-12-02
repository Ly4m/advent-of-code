use std::fs;

fn parse_input() -> Vec<usize> {
    fs::read_to_string("inputs/day_1.in")
        .unwrap()
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn solve_part_1() -> usize {
    0
}

pub fn solve_part_2() -> usize {
    0
}



#[cfg(test)]
mod tests {
    use crate::day_1;

    #[test]
    fn part_1() {

    }

    #[test]
    fn part_2() {

    }

}
