use std::fs;

#[derive(Debug)]
struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

fn parse_input(test_mode: bool, part: i8) -> Vec<Machine> {
    let path = if test_mode {
        format!("inputs_test/day_13_{}.in", part)
    } else {
        "inputs/day_13.in".to_owned()
    };

    let content = fs::read_to_string(path).unwrap();

    let lines: Vec<&str> = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut machines = Vec::new();

    for chunk in lines.chunks(3) {
        let button_a = parse_button_line(chunk[0]);
        let button_b = parse_button_line(chunk[1]);
        let prize = parse_prize_line(chunk[2]);

        let machine = Machine {
            button_a,
            button_b,
            prize,
        };

        machines.push(machine);
    }

    machines
}

fn parse_button_line(line: &str) -> (isize, isize) {
    let parts: Vec<&str> = line.split(": ").collect();
    let coordinates = parts[1].split(", ").collect::<Vec<&str>>();

    let x = coordinates[0].trim_start_matches("X+");
    let y = coordinates[1].trim_start_matches("Y+");

    (x.parse().unwrap(), y.parse().unwrap())
}

fn parse_prize_line(line: &str) -> (isize, isize) {
    let parts: Vec<&str> = line.split(": ").collect();
    let coordinates = parts[1].split(", ").collect::<Vec<&str>>();

    let x = coordinates[0].trim_start_matches("X=");
    let y = coordinates[1].trim_start_matches("Y=");

    (x.parse().unwrap(), y.parse().unwrap())
}

fn solve_machine(machine: &Machine, offset: isize) -> isize {
    let prize = (machine.prize.0 + offset, machine.prize.1 + offset);
    let determinant = machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0;
    let a = (prize.0 * machine.button_b.1 - prize.1 * machine.button_b.0) / determinant;
    let b = (machine.button_a.0 * prize.1 - machine.button_a.1 * prize.0) / determinant;

    if (machine.button_a.0 * a + machine.button_b.0 * b, machine.button_a.1 * a + machine.button_b.1 * b) == (prize.0, prize.1) {
        a * 3 + b
    } else {
        0
    }
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let claw_machines = parse_input(test_mode, 1);
    let x: isize = claw_machines.iter().map(|machine| solve_machine(machine, 0)).sum();
    x as usize
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let claw_machines = parse_input(test_mode, 1);
    let x: isize = claw_machines.iter().map(|machine| solve_machine(machine, 10000000000000)).sum();
    x as usize
}

#[cfg(test)]
mod tests {
    use crate::day_13::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 480);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 875318608908);
    }
}
