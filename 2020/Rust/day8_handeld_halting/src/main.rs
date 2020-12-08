use std::collections::HashSet;
use std::fs;

fn main() {
    let lines = fs::read_to_string("src/program.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let instructions = parse_into_instructions(&lines);
    run_instructions(&instructions);
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

struct Instruction {
    operation: String,
    argument: isize,
}

fn run_instructions(instructions: &Vec<Instruction>) {
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

    println!("{}", accumulator)
}
