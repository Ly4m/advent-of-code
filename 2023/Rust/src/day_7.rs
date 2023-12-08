use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Mul;

fn parse_input(test_mode: bool) -> impl Iterator<Item=String> {
    let path = if test_mode {
        "inputs_test/day_7.in".to_string()
    } else {
        "inputs/day_7.in".to_string()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok)
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: u32,
    hand_type: u32,
}

fn parse_hand(input: &str) -> Hand {
    let (hand_part, bid_part) = input.split_once(' ').unwrap();
    let mut counts: HashMap<char, u32> = HashMap::new();
    hand_part.chars().for_each(|c| *counts.entry(c).or_insert(0) += 1);
    let mut card_count: Vec<u32> = counts.values().cloned().collect();
    card_count.sort();
    card_count.reverse();

    let t = card_count.iter().map(|x| x.to_string()).collect::<String>();
    let hand_type = match t.as_str() {
        "5" => 6,
        "41" => 5,
        "32" => 4,
        "311" => 3,
        "221" => 2,
        "2111" => 1,
        "11111" => 0,
        _ => {
            panic!("Should not happen")
        }
    };

    let mut sorted_hand: String = String::from(hand_part);
    sorted_hand = sorted_hand.replace('A', "M");
    sorted_hand = sorted_hand.replace('K', "L");
    sorted_hand = sorted_hand.replace('Q', "K");
    sorted_hand = sorted_hand.replace('T', "I");
    sorted_hand = sorted_hand.replace('9', "H");
    sorted_hand = sorted_hand.replace('8', "G");
    sorted_hand = sorted_hand.replace('7', "F");
    sorted_hand = sorted_hand.replace('6', "E");
    sorted_hand = sorted_hand.replace('5', "D");
    sorted_hand = sorted_hand.replace('4', "C");
    sorted_hand = sorted_hand.replace('3', "B");
    sorted_hand = sorted_hand.replace('2', "A");


    Hand {
        bid: bid_part.parse().unwrap(),
        hand: sorted_hand,
        hand_type,
    }
}

fn parse_hand_with_jokers(input: &str) -> Hand {
    let (hand_part, bid_part) = input.split_once(' ').unwrap();
    let mut counts: HashMap<char, u32> = HashMap::new();
    hand_part.chars().for_each(|c| *counts.entry(c).or_insert(0) += 1);

    let number_of_joker = *counts.get(&'J').unwrap_or(&0);

    if number_of_joker > 0 && hand_part != "JJJJJ" {
        counts.remove(&'J');
        if let Some((&max_key, _)) = counts.iter().filter(|&(&k, _)| k != 'J').max_by_key(|&(_, &v)| v) {
            *counts.entry(max_key).or_insert(0) += number_of_joker;
        }
    }


    let mut card_count: Vec<u32> = counts.values().cloned().collect();
    card_count.sort();
    card_count.reverse();

    let t = card_count.iter().map(|x| x.to_string()).collect::<String>();

    let hand_type = match t.as_str() {
        "5" => 6,
        "41" => 5,
        "32" => 4,
        "311" => 3,
        "221" => 2,
        "2111" => 1,
        "11111" => 0,
        _ => {
            panic!("Should not happen")
        }
    };

    let mut sorted_hand: String = String::from(hand_part);
    sorted_hand = sorted_hand.replace('A', "N");
    sorted_hand = sorted_hand.replace('K', "M");
    sorted_hand = sorted_hand.replace('Q', "L");
    sorted_hand = sorted_hand.replace('T', "K");
    sorted_hand = sorted_hand.replace('9', "I");
    sorted_hand = sorted_hand.replace('8', "H");
    sorted_hand = sorted_hand.replace('7', "G");
    sorted_hand = sorted_hand.replace('6', "F");
    sorted_hand = sorted_hand.replace('5', "E");
    sorted_hand = sorted_hand.replace('4', "D");
    sorted_hand = sorted_hand.replace('3', "C");
    sorted_hand = sorted_hand.replace('2', "B");
    sorted_hand = sorted_hand.replace('J', "A");

    Hand {
        bid: bid_part.parse().unwrap(),
        hand: sorted_hand,
        hand_type,
    }
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let lines = parse_input(test_mode).collect::<Vec<String>>();
    let mut k = lines.iter().map(|l| parse_hand(l)).collect::<Vec<Hand>>();
    k.sort_by(|a, b| if b.hand_type.cmp(&a.hand_type) == Ordering::Equal {
        b.hand.cmp(&a.hand)
    } else { b.hand_type.cmp(&a.hand_type) });

    k.reverse();
    let mut sum: u32 = 0;
    for (i, el) in k.iter().enumerate() {
        sum += el.bid.mul(i as u32 + 1);
    }
    sum as usize
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let lines = parse_input(test_mode).collect::<Vec<String>>();
    let mut k = lines.iter().map(|l| parse_hand_with_jokers(l)).collect::<Vec<Hand>>();

    k.sort_by(|a, b| if b.hand_type.cmp(&a.hand_type) == Ordering::Equal {
        b.hand.cmp(&a.hand)
    } else { b.hand_type.cmp(&a.hand_type) });

    k.reverse();
    let mut sum: u32 = 0;
    for (i, el) in k.iter().enumerate() {
        sum += el.bid.mul(i as u32 + 1);
    }
    sum as usize
}

#[cfg(test)]
mod tests {
    use crate::day_7::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 6839);
    }
}
