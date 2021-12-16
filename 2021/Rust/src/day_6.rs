use std::collections::HashMap;
use std::fs;

fn parse_input(test_mode: bool) -> Vec<u8> {
    let path = if test_mode { "test_inputs/day_6.in" } else { "inputs/day_6.in" };
    fs::read_to_string(path).unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().expect("Parsing issue"))
        .collect()
}


pub fn solve_part_1(test_mode: bool) -> usize {
    let mut lanternfishes = parse_input(test_mode);

    for _x in 0..80 {
        lanternfishes = pass_one_day(&mut lanternfishes);
    }

    lanternfishes.len()
}

fn pass_one_day(lanternfishes: &mut Vec<u8>) -> Vec<u8> {
    let mut count = 0;
    let mut new_vec: Vec<u8> = vec![];
    for lanternfish in lanternfishes {
        match lanternfish {
            0 => {
                new_vec.push(6);
                count += 1;
            },
            1..=8 => {
                new_vec.push(*lanternfish - 1)
            }
            _ => panic!(),
        }
    }

    for _x in 0..count {
        new_vec.push(8);
    }

    new_vec
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let lanternfishes = parse_input(test_mode);

    let mut lanternfishes_count = HashMap::new();
    for x in 0..=8 {
        lanternfishes_count.insert(x as u8, 0);
    }

    for lanternfish in &lanternfishes {
        let old_value = *lanternfishes_count.get(lanternfish).expect("must be init");
        lanternfishes_count.insert(*lanternfish, old_value + 1);
    }

    for _day in 0..256 {
        let mut next_day: HashMap<u8, u64> = HashMap::new();
        for x in 0..=8 {
            next_day.insert(x as u8, 0);
        }

        for fishes in (0u8..=8u8).rev() {
            if (1u8..=8u8).contains(&fishes) {
                let value = lanternfishes_count.get(&fishes).expect("should have value");
                next_day.insert((fishes - 1) as u8, *value);
            }
        }

        let fishes_to_add = *lanternfishes_count.get(&0u8).expect("Should have value");
        let fishes_day_six = *next_day.get(&6u8).expect("Should have value");

        next_day.insert(6u8, fishes_day_six + fishes_to_add);
        next_day.insert(8u8, fishes_to_add);


        lanternfishes_count = next_day;
    }

    let val: u64 = lanternfishes_count.values().sum();
    val as usize
}

#[cfg(test)]
mod tests {
    use crate::day_6::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(5934, result)
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(26984457539, result)
    }
}
