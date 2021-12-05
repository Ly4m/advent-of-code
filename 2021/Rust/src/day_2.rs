use std::fs;

fn parse_input() -> Vec<String> {
    fs::read_to_string("inputs/day_2.in")
        .unwrap()
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn solve_part_1() -> usize {
    let inputs: Vec<String> = parse_input();
    calculate_destination(inputs)
}

pub fn solve_part_2() -> usize {
    let inputs: Vec<String> = parse_input();
    calculate_destination_with_aim(inputs)
}

fn calculate_destination(inputs: Vec<String>) -> usize {
    let mut depth: usize = 0;
    let mut horizontal_position: usize = 0;

    inputs.iter().for_each(|command| {
        let line = command.split_whitespace().collect::<Vec<&str>>();
        let (direction, value) = (line[0], line[1].parse::<usize>().expect("Wasn't a number"));

        match direction {
            "forward" => horizontal_position += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unknwon Instruction"),
        }
    });

    depth * horizontal_position
}

pub fn calculate_destination_with_aim(inputs: Vec<String>) -> usize {
    let mut aim: usize = 0;
    let mut depth: usize = 0;
    let mut horizontal_position: usize = 0;

    inputs.iter().for_each(|command| {
        let line = command.split_whitespace().collect::<Vec<&str>>();
        let (direction, value) = (line[0], line[1].parse::<usize>().expect("Wasn't a number"));

        match direction {
            "forward" => {
                horizontal_position += value;
                depth += value * aim;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Unknwon Instruction"),
        }
    });

    depth * horizontal_position
}


#[cfg(test)]
mod tests {
    use crate::day_2::{calculate_destination, calculate_destination_with_aim};

    #[test]
    fn part_1() {
        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ].iter().map(|x| x.to_string()).collect();

        let result = calculate_destination(data);
        assert_eq!(150, result)
    }

    #[test]
    fn part_2() {
        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ].iter().map(|x| x.to_string()).collect();

        let result = calculate_destination_with_aim(data);
        assert_eq!(900, result)
    }

}
