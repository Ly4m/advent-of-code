use std::fs;
use std::collections::HashSet;

fn main() {
    let lines = fs::read_to_string("src/data.in").unwrap().lines()
        // .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().expect("Not a number found"))
        .collect::<Vec<usize>>();


    let part_one = solve_part_1(&lines, 25);
    println!("{}", part_one);

    let part_two = solve_part_2(&lines, *part_one);
    println!("{}", part_two);

}

fn solve_part_2(numbers: &Vec<usize>, target: usize) -> usize{


    for (index, number) in numbers.iter().enumerate() {
        let mut current_streak = HashSet::new();

        let mut aggregator = *number;

        for inner_number in numbers.iter().skip(index + 1) {

            if aggregator + inner_number > target {
                current_streak.drain();
                break;
            }

            if aggregator + inner_number == target {
                current_streak.insert(inner_number);
                println!("Found sum {} for {:?}", target, current_streak);
                return *current_streak.iter().min().unwrap() + *current_streak.iter().max().unwrap();
            }

            current_streak.insert(inner_number);
            aggregator += *inner_number;

        }
    }

    0
}

fn solve_part_1(lines: &Vec<usize>, preamble_size: usize) -> &usize {
    for (index, number) in lines.iter().enumerate().skip(preamble_size) {
        if !two_sum_exists(&lines.iter().skip(index - preamble_size).take(preamble_size)
            .collect::<Vec<&usize>>(), number) {
            return number;
        }
    };
    &0
}

fn two_sum_exists(report: &Vec<&usize>, &year: &usize) -> bool {
    for (index, value) in report.iter().enumerate() {
        for value2 in report[index..report.len()].iter() {
            if *value + *value2 == year {
                return true;
            }
        }
    }
    false
}
