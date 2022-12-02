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
    let map = parse_input(test_mode);
    let mut lowest_points= vec![];

    for (y, line) in map.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if is_lowest_point(&map, (y, x)) {
                lowest_points.push(&map[y][x]);
            }
        }
    }
    lowest_points.iter().map(|x| (*x + 1) as usize).sum()
}

fn is_lowest_point(table: &[Vec<u8>], indexes: (usize, usize)) -> bool {
    let y_start = if indexes.0 > 0 { indexes.0-1 } else { 0 };
    let y_end = if indexes.0 < (table.len() - 1) { indexes.0+1 } else { table.len() - 1 };

    let x_start = if indexes.1 > 0 { indexes.1-1 } else { 0 };
    let x_end = if indexes.1 < (table[0].len() - 1) { indexes.1+1 } else { table[0].len() - 1 };

    let min = *table.iter().skip(y_start).take(y_end + 1)
        .flat_map(|x| x.iter().skip(x_start).take(x_end + 1))
        .min().expect("Min could not be found");

    min == table[indexes.0][indexes.1]
}

pub fn solve_part_2(_test_mode: bool) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_9::{solve_part_1, solve_part_2, is_lowest_point, parse_input};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(15, result)
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(1134, result)
    }

    #[test]
    fn should_be_lowest_point_on_border() {
        let data = parse_input(true);
        let result = is_lowest_point(&data, (0, 1));
        assert_eq!(true, result)
    }
    #[test]
    fn should_be_lowest_point() {
        let data = parse_input(true);
        let result = is_lowest_point(&data, (2, 2));
        assert_eq!(true, result)
    }

    #[test]
    fn should_not_be_lowest_point() {
        let data = parse_input(true);
        let result = is_lowest_point(&data, (3, 2));
        assert_eq!(false, result)
    }
}
