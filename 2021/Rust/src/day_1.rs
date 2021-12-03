use std::fs;

fn parse_input() -> Vec<usize> {
    fs::read_to_string("inputs/day_1.in")
        .unwrap()
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn solve_part_1() -> usize {
    let sonar_input: Vec<usize> = parse_input();
    count_depth_measurement_increases(sonar_input)
}

pub fn solve_part_2() -> usize {
    let sonar_input: Vec<usize> = parse_input();
    count_depth_measurement_increases_by_three(sonar_input)
}

fn count_depth_measurement_increases(measurements: Vec<usize>) -> usize {
    measurements.windows(2).filter(|x| x[1] > x[0]).count()
}

fn count_depth_measurement_increases_by_three(measurements: Vec<usize>) -> usize {
    let depth_by_three: Vec<usize> = measurements.windows(3).map(|x| x.iter().sum()).collect();
    depth_by_three.windows(2).filter(|x| x[1] > x[0]).count()
}


#[cfg(test)]
mod tests {
    use crate::day_1::{count_depth_measurement_increases, count_depth_measurement_increases_by_three};

    #[test]
    fn part_1() {
        let data: Vec<usize>  = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = count_depth_measurement_increases(data);
        assert_eq!(7, result)
    }

    #[test]
    fn part_2() {
        let data: Vec<usize>  = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = count_depth_measurement_increases_by_three(data);
        assert_eq!(5, result)
    }

}
