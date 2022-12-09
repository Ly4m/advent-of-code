use std::collections::HashSet;
use std::fs;

fn parse_input(test_mode: bool) -> Vec<String> {
    let path = if test_mode { "inputs_test/day_3.in" } else { "inputs/day_3.in" };
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let rucksacks = parse_input(test_mode);
    rucksacks.iter().map(|rucksack| {
        let part_1 = &rucksack[..rucksack.len() / 2];
        let part_2 = &rucksack[rucksack.len() / 2..rucksack.len()];
        let shared_item = intersection(part_1, part_2);
        compute_priority(shared_item)
    }).sum()
}

fn intersection(part_1: &str, part_2: &str) -> char {
    let set: HashSet<char> = part_1.chars().collect();
    let char = part_2.chars().find(|c| set.contains(&c));
    char.unwrap()
}

fn compute_priority(item: char) -> usize {
    let lower_case = ('a'..='z').into_iter().collect::<Vec<char>>();
    let upper_case = ('A'..='Z').into_iter().collect::<Vec<char>>();
    if item.is_lowercase() {
        lower_case.iter().position(|c| c == &item).unwrap() + 1
    } else {
        upper_case.iter().position(|c| c == &item).unwrap() + 27
    }
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let mut rucksacks = parse_input(test_mode);

    rucksacks.chunks_mut(3).map(|group| {
        let badge = group_intersection(group);
        compute_priority(badge)
    }).sum()
}

fn group_intersection(elves: &mut [String]) -> char {
    elves.sort_by(|elf_1, elf_2| elf_1.len().partial_cmp(&elf_2.len()).unwrap());
    let elf_2_bag: HashSet<char> = elves[1].chars().collect();
    let elf_3_bag: HashSet<char> = elves[2].chars().collect();

    let badge = elves[0].chars().find(|c| elf_2_bag.contains(c) && elf_3_bag.contains(c));
    badge.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_3::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 157);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 70);
    }
}
