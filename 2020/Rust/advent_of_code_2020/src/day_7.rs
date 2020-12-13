use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

fn parse_input() -> Vec<String> {
    fs::read_to_string("inputs/day_7.in").unwrap().lines()
        .map(String::from)
        .map(|s| s.replace(".", "").replace(" bags", "").replace(" bag", ""))
        .collect()
}

pub fn solve_part_2() -> usize {
    let lines = parse_input();
    let rules = parse_rules(&lines);
    compute_sub_bags("shiny gold", &rules) - 1
}

fn parse_rules(lines: &Vec<String>) -> HashMap<&str, Vec<&str>> {
    HashMap::from_iter(
        lines
            .iter()
            .map(|rule| -> (&str, Vec<&str>)  {
                let mut splited_rule = rule.split(" contain ");
                let bag_name = splited_rule.next().unwrap();
                let bag_content = splited_rule.next().map(|sub_bag| sub_bag.split(", ").collect::<Vec<&str>>()).unwrap();
                (bag_name, bag_content)
            }))
}

fn compute_sub_bags(current_rule: &str, all_rules: &HashMap<&str, Vec<&str>>) -> usize {
    let sub_bags = all_rules.get(current_rule).unwrap();

    if sub_bags.contains(&"no other") {
        return 1
    }

    sub_bags
        .iter()
        .map(|bag| {
            let bag_array = bag.split(" ").collect::<Vec<&str>>();
            (bag_array[0].parse::<usize>().unwrap(), format!("{} {}", bag_array[1], bag_array[2]))
        })
        .map(|(count, bag_name)| count * compute_sub_bags(&bag_name[..], &all_rules))
        .sum::<usize>() + 1
}

pub fn solve_part_1() -> usize {
    let lines = &parse_input();
    let rules = parse_rules(lines);

    let mut bags_to_parse: HashSet<&str> = HashSet::new();
    bags_to_parse.insert("shiny gold");

    let mut parent_bags: HashSet<&str> = HashSet::new();

    loop {
        let mut found_bags: HashSet<&str> = HashSet::new();

        for (bag_name, bag_content) in &rules {
            if bag_content.contains(&"no other") { continue }

            let current_bag: HashSet<&str> = HashSet::from_iter(bag_content.iter().map(|x| *x));

            if current_bag.iter().any(|bag| bags_to_parse.iter().any(|sub_bag| bag.contains(sub_bag))) {
                parent_bags.insert(*bag_name);
                found_bags.insert(*bag_name);
            }
        }

        bags_to_parse = found_bags.clone(); // add newfound bags in the list of bags to parse

        if bags_to_parse.is_empty() { break; } // found all necessary bags
    }

    parent_bags.len()
}
