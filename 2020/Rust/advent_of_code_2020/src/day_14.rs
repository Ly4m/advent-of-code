use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
struct Instruction {
    index: usize,
    value: usize,
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("inputs/day_14.in").expect("Missing input file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}

fn parse_instruction_with_binary_value(line: &str) -> (usize, String) {
    let split = line.split(" = ").collect::<Vec<&str>>();
    let address_split = split.get(0).expect("missing address").to_owned();
    let value_split = split.get(1).expect("missing value");

    let address = address_split[4..address_split.len() - 1].parse::<usize>().expect("Cannot parse address");
    let value = format!("{:036b}", value_split.parse::<usize>().expect("cannot parse value"));

    (address, value)
}

fn parse_instruction_with_binary_address(line: &str) -> (String, usize) {
    let split = line.split(" = ").collect::<Vec<&str>>();
    let address_split = split.get(0).expect("missing address").to_owned();
    let value_split = split.get(1).expect("missing value");

    let address = address_split[4..address_split.len() - 1].parse::<usize>().expect("Cannot parse address");
    let value = value_split.parse::<usize>().expect("cannot parse value");

    (format!("{:036b}", address), value)
}

pub fn solve_part_1() -> usize {
    let program = parse_input();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    let mut current_mask: String = String::new();
    for instruction in program {
        if instruction.starts_with("mask") {
            current_mask = String::from(instruction.split_at(7).1);
        } else {
            let (address, value) = parse_instruction_with_binary_value(instruction.as_str());
            let new_value = apply_mask_to_value(value.as_str(), &current_mask);
            memory.insert(address, new_value);
        }
    };

    memory.values().sum()
}

fn apply_mask_to_value(base_value: &str, mask: &str) -> usize {
    let new_value_binary: String = mask.chars().enumerate().map(|(index, char)|
        if char == 'X' {
            base_value.chars().nth(index).expect("parsing issue")
        } else {
            char
        }).collect::<String>();

    usize::from_str_radix(new_value_binary.as_str(), 2).expect("Cannot parse number")
}


pub fn solve_part_2() -> usize {
    let program = parse_input();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    let mut current_mask: String = String::new();
    for instruction in program {
        if instruction.starts_with("mask") {
            current_mask = String::from(instruction.split_at(7).1);
        } else {
            let (address, value) = parse_instruction_with_binary_address(instruction.as_str());
            let unmasked_address = apply_mask_to_address(&current_mask, address);
            let addresses = compute_floating_address(unmasked_address);
            for floating_address in addresses.iter() {
                memory.insert( usize::from_str_radix(floating_address.as_str(), 2).expect("Cannot parse number"), value);
            }
        }
    };

    memory.values().sum()
}

fn apply_mask_to_address(mask: &str, base_value: String) -> String {
    mask.chars().enumerate().map(|(index, char)|
        if char == '0' {
            base_value.chars().nth(index).expect("parsing issue")
        } else {
            char
        }).collect::<String>()
}

fn compute_floating_address(floating_address: String) -> Vec<String> {
    if floating_address.is_empty() {
        return vec![String::from("")];
    }

    if floating_address.len() == 1 {
        return if floating_address.starts_with('X') {
            vec![String::from("0"), String::from("1")]
        } else {
            vec![floating_address]
        };
    }

    let (first_char, rest_of_address) = floating_address.split_at(1);

    if first_char == "X" {
        let mut one: Vec<String> = compute_floating_address(String::from(rest_of_address)).iter().map(|x| String::from("0") + x).collect();
        let zero: Vec<String> = compute_floating_address(String::from(rest_of_address)).iter().map(|x| String::from("1") + x).collect();
        one.extend(zero);
        one
    } else {
        compute_floating_address(String::from(rest_of_address)).iter().map(|x| String::from(first_char) + x).collect()
    }

}


#[cfg(test)]
mod tests {
    use crate::day_14::{apply_mask_to_value, parse_instruction_with_binary_address, parse_instruction_with_binary_value, solve_part_1, solve_part_2, compute_floating_address};

    #[test]
    fn test_compute_floating_address_1() {
        assert_eq!(vec![String::from("0"), String::from("1")], compute_floating_address(String::from("X")))
    }

    #[test]
    fn test_compute_floating_address_2() {
        assert_eq!(vec![String::from("00"), String::from("01")], compute_floating_address(String::from("0X")))
    }

    #[test]
    fn test_compute_floating_address_3() {
        let addresses = compute_floating_address(String::from("XX"));
        assert_eq!(vec![String::from("00"), String::from("01"), String::from("10"), String::from("11")], addresses);
    }

    #[test]
    fn test_parse_instruction_with_binary_value() {
        assert_eq!((8, String::from("000000000000000000000000000000001011")), parse_instruction_with_binary_value("mem[8] = 11"));
        assert_eq!((77, String::from("000000000000000000000000000001100101")), parse_instruction_with_binary_value("mem[77] = 101"));
        assert_eq!((8, String::from("000000000000000000000000000000000000")), parse_instruction_with_binary_value("mem[8] = 0"));
    }

    #[test]
    fn test_parse_instruction_with_binary_address() {
        assert_eq!((String::from("000000000000000000000000000000001011"), 8), parse_instruction_with_binary_address("mem[11] = 8"));
        assert_eq!((String::from("000000000000000000000000000001100101"), 77), parse_instruction_with_binary_address("mem[101] = 77"));
        assert_eq!((String::from("000000000000000000000000000000000000"), 8), parse_instruction_with_binary_address("mem[0] = 8"));
    }

    #[test]
    fn test_apply_mask() {
        assert_eq!(73, apply_mask_to_value(&"000000000000000000000000000000001011", &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
        assert_eq!(101, apply_mask_to_value(&"000000000000000000000000000001100101", &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
        assert_eq!(64, apply_mask_to_value(&"000000000000000000000000000000000000", &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(13105044880745, solve_part_1())
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3505392154485, solve_part_2())
    }
}
