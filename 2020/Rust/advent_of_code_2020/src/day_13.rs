use std::fs;

fn parse_input() -> (usize, Vec<usize>) {
    let lines: Vec<String> = fs::read_to_string("inputs/day_13.in").expect("Missing input file")
        .lines()
        .map(String::from)
        .collect();

    let time = lines.get(0).map(|x| x.parse::<usize>()).expect("Time missing").unwrap();
    let buses = lines.get(1).map(|line| line.split(",")
        .filter(|x| x != &"x").map(|id| id.parse::<usize>().expect("Not a number"))
        .collect::<Vec<usize>>()).expect("Missing bus IDs");

    (time, buses)
}

fn parse_input_part_2() -> Vec<usize> {
    let lines: Vec<String> = fs::read_to_string("inputs/day_13.in").expect("Missing input file")
        .lines()
        .map(String::from)
        .collect();

    lines.get(1).map(|line| line.split(",").map(|x| if x == "x" { 0 } else { x.parse::<usize>().unwrap() }).collect::<Vec<usize>>()).expect("MCSTN")
}

pub fn solve_part_1() -> usize {
    let (departure_time, buses) = parse_input();
    let max_wait_time = departure_time + *buses.iter().max().unwrap();
    let mut bus_id: usize = 0;
    let mut time_waited: usize = 0;


    for time in departure_time..max_wait_time {
        let bus = buses.iter().find(|bus| &time % *bus == 0);

        if bus.is_some() {
            bus_id = *bus.unwrap();
            time_waited = &time - &departure_time;
            break;
        }
    }

    bus_id * time_waited
}

pub fn solve_part_2() -> usize {
    let inputs = parse_input_part_2();

    let mut current_time = 0;
    let mut step = *inputs.get(0).unwrap();

    // had to deref here I don't like it + skip first value
    for (index, bus) in inputs.iter().enumerate().map(|(x, y)| (x, *y)).skip(1) {
        if bus == 0 { continue } // Workaround to manage X's

        loop {
            if ((&current_time + index) % bus) == 0 { // if current time + time offset is a multiple of the bus
                step = step * bus;
                break;
            }
            current_time += step; // increment by step
        }
    }

    current_time
}


#[cfg(test)]
mod tests {
    use crate::day_13::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(3464, solve_part_1())
    }

    #[test]
    fn test_part_2() {
        assert_eq!(760171380521445, solve_part_2())
    }
}
