use std::fs;
use std::ops::Sub;
use std::collections::HashMap;

fn parse_input() -> Vec<usize> {
    fs::read_to_string("inputs/day_10.in").unwrap().lines()
        .map(|x| x.parse::<usize>().expect("Not a number found"))
        .collect::<Vec<usize>>()
}

pub fn solve_part_1() -> usize {
    let adapters = parse_input();

    let mut sorted_adapter: Vec<usize> = adapters;
    sorted_adapter.sort();

    sorted_adapter.push(sorted_adapter.iter().max().unwrap() + 3);

    let mut previous = 0;

    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;


    for adapter in sorted_adapter {
        let diff = adapter - previous;

        match diff {
            1 => one_jolt_diff += 1,
            3 => three_jolt_diff += 1,
            _ => (),
        };

        previous = adapter;
    }

    one_jolt_diff * three_jolt_diff
}

pub fn solve_part_2() -> usize {
    let mut adapters = parse_input();
    adapters.push(0); //  Add lower limit

    let mut sorted_adapter: Vec<usize> = adapters;
    sorted_adapter.sort();

    sorted_adapter.push(sorted_adapter.iter().max().unwrap() + 3); // add upper limit

    let mut already_computed: HashMap<usize, usize> = HashMap::new();

    parse_tree(&sorted_adapter, 0, &mut already_computed)
}

fn parse_tree(adapters: &[usize], index: usize, already_computed: &mut HashMap<usize, usize>) -> usize {
    if already_computed.contains_key(&index) {
        return *already_computed.get(&index).unwrap();
    }

    if index == adapters.len() - 1 {
        return 1;
    } // Leaf

    let current_adapter_value = adapters.get(index).expect("Overflow, should not happen");
    let mut current_adapter_combinations = 0;


    for i in 1..=3 as usize {
        let next_index = index + i;

        if next_index >= adapters.len() { continue }

        if adapters.get(next_index).map(|next_value| next_value.sub(current_adapter_value).le(&3)).ok_or(false).unwrap_or(false) {
            current_adapter_combinations += parse_tree(adapters, next_index, already_computed);
        }
    }

    already_computed.insert(index, current_adapter_combinations);
    current_adapter_combinations
}

