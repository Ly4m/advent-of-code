use std::fs;

fn main() {
    let adapters = fs::read_to_string("src/adapters.in").unwrap().lines()
        .map(|x| x.parse::<usize>().expect("Not a number found"))
        .collect::<Vec<usize>>();

    let part_1 = solve_part_1(&adapters);
    println!("{}", part_1)
}


fn solve_part_1(adapters: &Vec<usize>) -> usize {
    let mut sorted_adapter: Vec<usize> = adapters.clone();
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


    println!("One diff found : {}", one_jolt_diff);
    println!("Three diff found : {}", three_jolt_diff);

    one_jolt_diff * three_jolt_diff
}
