use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
#[derive(Debug)]
struct Program {
    mask: String,
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
struct Instruction {
    index: usize,
    value: usize,
}

fn parse_input() -> Vec<Program> {
    let mut lines: Vec<String> = fs::read_to_string("inputs/day_14.in").expect("Missing input file")
        .lines()
        .map(String::from)
        .collect();

    lines.push(String::from(""));

    let mut chunks: Vec<Vec<String>> = vec![];
    let mut buffer: Vec<String> = vec![];
    buffer.push(lines.get(0).unwrap().clone());

    for line in lines.iter().skip(1) {
        if line.starts_with("mask") || line.is_empty() {
            chunks.push(buffer.clone());
            buffer = vec![];
        }
        buffer.push(line.clone());
    }
    chunks.iter().map(|x| parse_program(x)).collect()
}

fn parse_program(chunk: &[String]) -> Program {
    let mask = String::from(chunk.get(0).expect("Missing mask").split_at(7).1);
    let instructions = chunk.iter().skip(1).map(|x| parse_instruction(x.as_str())).collect::<Vec<Instruction>>();

    Program { mask, instructions }
}

fn parse_instruction(line: &str) -> Instruction {
    let split = line.split(" = ").collect::<Vec<&str>>();
    Instruction {
        index: split.get(0).unwrap().split_at(4).1.split("]").collect::<Vec<&str>>().get(0).map(|x| x.parse::<usize>()).unwrap().unwrap(),
        value: split.get(1).map(|x| x.parse().unwrap()).unwrap(),
    }
}


pub fn solve_part_1() -> usize {
    let programs = parse_input();
    let mut values: HashMap<usize, usize> = HashMap::new();

    for program in programs.iter() {

        for instruction in &program.instructions {
            let value_as_binary = format!("{:036b}", instruction.value);
            let new_value = program.mask.chars().to_owned().enumerate().map(|(index, x)| {
                match x {
                    'X' => value_as_binary.chars().nth(index).expect("parsing issue"),
                    '0' => '0',
                    '1' => '1',
                    _ => panic!("unexpected char")
                }
            }).collect::<String>();

            values.insert(instruction.index, usize::from_str_radix(new_value.as_str(), 2).expect("Cannot parse binary"));
        }
    };

    values.values().sum()
}


pub fn solve_part_2() -> usize {
    0
}


#[cfg(test)]
mod tests {
    use crate::day_14::{parse_instruction, solve_part_1};

    #[test]
    fn test_parse_instruction() {
        assert_eq!(11, parse_instruction("mem[8] = 11").value);
        assert_eq!(8, parse_instruction("mem[8] = 11").index);
    }


    #[test]
    fn test_part_1() {
        assert_eq!(165, solve_part_1())
    }
}
