use io::BufReader;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let path = if test_mode {
        format!("inputs_test/day_5_{}.in", part)
    } else {
        "inputs/day_5.in".to_owned()
    };

    let file = File::open(path).unwrap();
    let sleigh_launch_safety_manual = BufReader::new(file);

    let mut page_ordering_rules: Vec<(i32, i32)> = Vec::new();
    let mut updates_pages: Vec<Vec<i32>> = Vec::new();

    let mut parsing_ordering_rules = true;

    for line in sleigh_launch_safety_manual.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            parsing_ordering_rules = false;
            continue;
        }

        if parsing_ordering_rules {
            let parts: Vec<i32> = line
                .split('|')
                .map(|page_num| page_num.parse::<i32>().unwrap())
                .collect();
            page_ordering_rules.push((parts[0], parts[1]));
        } else {
            let update_pages: Vec<i32> = line
                .split(',')
                .map(|page_num| page_num.parse::<i32>().unwrap())
                .collect();
            updates_pages.push(update_pages);
        }
    }

    (page_ordering_rules, updates_pages)
}

fn find_middle_page(update: &[i32]) -> usize {
    update[update.len() / 2] as usize
}

fn create_rules_as_map(page_ordering_rules: &[(i32, i32)]) -> HashMap<i32, Vec<i32>> {
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for &(x, y) in page_ordering_rules {
        rules_map.entry(x).or_default().push(y);
    }
    rules_map
}

fn is_correct_order(update: &[i32], rules_map: &HashMap<i32, Vec<i32>>) -> bool {
    let page_num_by_index: HashMap<i32, usize> = update
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    rules_map.iter().all(|(&page_num, pages_after)| {
        page_num_by_index
            .get(&page_num)
            .map_or(true, |&current_page_index| {
                pages_after.iter().all(|&page_after| {
                    page_num_by_index
                        .get(&page_after)
                        .map_or(true, |&page_after_index| {
                            current_page_index < page_after_index
                        })
                })
            })
    })
}

fn correct_order_comparator(a: &i32, b: &i32, rules_map: &HashMap<i32, Vec<i32>>) -> Ordering {
    if let Some(pages_after) = rules_map.get(a) {
        if pages_after.contains(b) {
            return Ordering::Less;
        }
    }

    if let Some(pages_after) = rules_map.get(b) {
        if pages_after.contains(a) {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let (page_ordering_rules, updates_pages) = parse_input(test_mode, 1);
    let rules_as_map = create_rules_as_map(&page_ordering_rules);

    updates_pages
        .iter()
        .filter(|update| is_correct_order(update, &rules_as_map))
        .map(|update| find_middle_page(update))
        .sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let (page_ordering_rules, updates) = parse_input(test_mode, 1);
    let rules_map = create_rules_as_map(&page_ordering_rules);
    
    let mut total_middle_sum = 0;
    
    let mut incorrect_updates: Vec<Vec<i32>> = updates
        .into_iter()
        .filter(|update| !is_correct_order(update, &rules_map))
        .collect();

    incorrect_updates.iter_mut().for_each(|index| {
        index.sort_by(|a, b| correct_order_comparator(a, b, &rules_map));
    });

    incorrect_updates
        .iter()
        .map(|update| find_middle_page(update))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_5::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 143);
    }
    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 123);
    }
}
