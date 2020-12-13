use std::collections::HashSet;
use std::fs;

fn parse_input() -> Vec<Instruction> {
    let lines = fs::read_to_string("inputs/day_8.in").unwrap().lines()
        .map(String::from)
        .collect();
    parse_into_instructions(&lines)
}

#[derive(Clone)]
#[derive(Debug)]
struct Instruction {
    operation: String,
    argument: isize,
}

fn parse_into_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .map(|line| -> Instruction {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            Instruction {
                operation: String::from(split_line[0]),
                argument: split_line[1].parse::<isize>().expect("Wasn't a number"),
            }
        })
        .collect()
}

pub fn solve_part_2() -> usize {
    let instructions = &parse_input();
    let instruction_count = instructions
        .iter()
        .map(|instruction| if instruction.operation.as_str() != "acc" { true } else { false })
        .collect::<Vec<bool>>();

    for (index, to_mutate) in instruction_count.iter().enumerate() {
        if *to_mutate {
            let mut clone = instructions.to_vec();
            let Instruction { operation, argument } = &clone[index];
            let swapped_operation = if operation.as_str() == "jmp" { "nop" } else { "jmp" };

            clone[index] = Instruction {
                operation: String::from(swapped_operation),
                argument: *argument,
            };

            let result = run_program(&clone);

            if result != 0 { return result; }
        }
    }

    0 // No result found !
}

fn run_program(instructions: &Vec<Instruction>) -> usize {
    let mut accumulator: isize = 0;
    let mut index: isize = 0;

    let mut all_index: HashSet<isize> = (0..instructions.len() as isize).collect();

    loop {
        let Instruction { operation, argument } = instructions.get(index as usize).expect("No instruction found");

        match operation.as_str() {
            "acc" => {
                accumulator += argument;
                index += 1;
            }
            "nop" => index += 1,
            "jmp" => index += argument,
            _ => panic!("Unknwon Instruction"),
        }

        if index == instructions.len() as isize {
            return accumulator as usize;
        }

        if !all_index.remove(&index) {
            break;
        }
    }

    0
}

pub fn solve_part_1() -> usize {
    let instructions = &parse_input();
    let mut accumulator: isize = 0;
    let mut index: isize = 0;

    let mut all_index: HashSet<isize> = (0..instructions.len() as isize).collect();

    loop {
        let Instruction { operation, argument } = instructions.get(index as usize).expect("No instruction found");

        match operation.as_str() {
            "acc" => {
                accumulator += argument;
                index += 1;
            }
            "nop" => index += 1,
            "jmp" => index += argument,
            _ => panic!("Unknwon Instruction"),
        }

        if !all_index.remove(&index) {
            break;
        }
    }

    accumulator as usize
}
