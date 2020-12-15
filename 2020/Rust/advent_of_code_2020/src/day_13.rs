use std::fs;

fn parse_input() -> (usize, Vec<usize>) {
    let lines: Vec<String> = fs::read_to_string("inputs/day_13.in").expect("Missing input file")
        .lines()
        .map(String::from)
        .collect();

    let time = lines.iter().get(0).map(|x| x.parse::<usize>()).expect("Time missing").unwrap();
    let buses = lines.get(1).map(|line| line.split(",")
        .filter(|x| x != &"x").map(|id| id.parse::<usize>().expect("Not a number"))
        .collect::<Vec<usize>>()).expect("Missing bus IDs");

    (time, buses)
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

    0
}


#[cfg(test)]
mod tests {
    use crate::day_13::solve_part_1;

    #[test]
    fn test_part_1() {
        assert_eq!(3464, solve_part_1())
    }
}
