use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: &str) -> impl Iterator<Item=String> {
    const TEST_INPUT_PATH: &str = "inputs_test/";
    const INPUT_PATH: &str = "inputs/";

    let path = if test_mode { TEST_INPUT_PATH.to_string() + "day_8_" + part + ".in" } else { INPUT_PATH.to_string() + "day_8_" + part + ".in" };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

// fn traverse_graph(direction: &str, network: &HashMap<&str, (&str, &str)>) {
//     network.get(direction).unwrap();
// }

pub fn solve_part_1(test_mode: bool) -> usize {
    let mut lines = parse_input(test_mode, "1");
    let directions = lines.next().unwrap();
    lines.next();
    let mut network: HashMap<String, (String, String)> = HashMap::new();

    for node in lines {
        let node_parts = node.split_once('=').unwrap();
        let node_id = node_parts.0.trim().to_string();
        let links = node_parts.1.trim()
            .strip_prefix('(').unwrap()
            .strip_suffix(')').unwrap()
            .split_once(", ").unwrap();

        network.insert(node_id, (links.0.to_string(), links.1.to_string()));
    }
    let starting_nodes: Vec<String> = network.keys().filter(|k| k.ends_with('Z')).map(|c| c.to_string()).collect();
    dbg!(starting_nodes);
    let mut step = 0;
    let mut current_node = String::from("AAA");

    while current_node != "ZZZ" {
        for x in directions.chars() {
            match x {
                'R' => {
                    current_node = network.get(&current_node).unwrap().1.clone();
                }
                'L' => {
                    current_node = network.get(&current_node).unwrap().0.clone();
                }
                _ => panic!("Not expected")
            }
            step += 1;
            if current_node == "ZZZ" {
                break;
            }
        }
    }

    step
}


fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: Vec<usize>) -> usize {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let mut lines = parse_input(test_mode, "2");
    let directions = lines.next().unwrap();
    lines.next();
    let mut network: HashMap<String, (String, String)> = HashMap::new();

    for node in lines {
        let node_parts = node.split_once('=').unwrap();
        let node_id = node_parts.0.trim().to_string();
        let links = node_parts.1.trim()
            .strip_prefix('(').unwrap()
            .strip_suffix(')').unwrap()
            .split_once(", ").unwrap();

        network.insert(node_id, (links.0.to_string(), links.1.to_string()));
    }
    let starting_nodes: Vec<String> = network.keys().filter(|k| k.ends_with('A')).map(|c| c.to_string()).collect();
    let mut steps: Vec<usize> = vec![];

    for mut current in starting_nodes {
        for (step, direction) in directions.chars().cycle().enumerate() {
            if current.ends_with('Z') {
                steps.push(step);
                break;
            }
            let node = network.get(&current).unwrap();
            match &direction {
                'L' => current = node.0.clone(),
                'R' => current = node.1.clone(),
                _ => (),
            }
        }
    }

    lcm_of_vec(steps)
}

#[cfg(test)]
mod tests {
    use crate::day_8::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 6);
    }
}
