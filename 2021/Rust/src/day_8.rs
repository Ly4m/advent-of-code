use std::collections::HashMap;
use std::fs;

fn parse_input(test_mode: bool) -> Vec<(Vec<String>, Vec<String>)> {
    let path = if test_mode { "test_inputs/day_8.in" } else { "inputs/day_8.in" };
    fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| {
            let mut line_split = line.split('|');
            (line_split.next().unwrap().split_whitespace().map(|x| x.to_string()).collect(), line_split.next().unwrap().split_whitespace().map(|x| x.to_string()).collect())
        })
        .collect()
}


pub fn solve_part_1(test_mode: bool) -> usize {
    let toto = parse_input(test_mode);
    toto.iter()
        .map(|x| x.1.clone())
        .map(|x| {
            x.iter().filter(|y| [2, 3, 4, 7].contains(&(y.len() as i32))).count()
        })
        .sum()
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let toto = parse_input(test_mode);

    let mut output_sum = 0;

    for (input, output) in toto.iter() {
        let mut translation_map: HashMap<usize, String> = HashMap::new();

        for value in input {
            match value.len() {
                2 => translation_map.insert(1, value.clone()),
                3 => translation_map.insert(7, value.clone()),
                4 => translation_map.insert(4, value.clone()),
                7 => translation_map.insert(8, value.clone()),
                _ => None
            };
        }

        let six = input.iter()
            .filter(|x| x.len() == 6)
            .find(|x| translation_map.get(&1).unwrap().clone().chars().any(|c1| !x.chars().any(|y| y == c1)))
            .expect("Six not found");

        let zero = input.iter()
            .filter(|x| x.len() == 6)
            .filter(|x| *x != six)
            .find(|x| translation_map.get(&4).unwrap().clone().chars().any(|c1| !x.chars().any(|y| y == c1)))
            .expect("Zero not found");


        //find 6 and 9
        let one = translation_map.get(&1).unwrap().clone();
        let nine = input.iter()
            .filter(|x| x.len() == 6)
            .find(|x| *x != zero && *x != six)
            .expect("missing 9");

        translation_map.insert(0, zero.clone());
        translation_map.insert(6, six.clone());
        translation_map.insert(9, nine.clone());

        // 3
        let three = input.iter()
            .filter(|x| x.len() == 5)
            .find(|x| one.chars().all(|y| x.chars().any(|z| y == z)))
            .expect("missing 3");


        //find middle
        let middle = *diff_string(translation_map.get(&8).unwrap(), translation_map.get(&0).unwrap())
            .get(0).expect("Middle segment not found");

        //find top left
        let top_left = *diff_string(translation_map.get(&4).unwrap(), translation_map.get(&1).unwrap())
            .iter().find(|x| **x != middle).expect("Top left segment not found");

        let five = input.iter()
            .filter(|x| x.len() == 5)
            .filter(|x| *x != three)
            .find(|x| x.chars().any(|y| y == top_left))
            .expect("missing 5");

        let two = input.iter()
            .find(|x| x.len() == 5 && *x != five && *x != three)
            .expect("missing 2");

        translation_map.insert(3, three.clone());
        translation_map.insert(2, two.clone());
        translation_map.insert(5, five.clone());

        output_sum += translate_output(output, &translation_map);
    }

    output_sum
}

fn translate_output(output: &[String], translation_map: &HashMap<usize, String>) -> usize {
    let values: Vec<String> = output.iter().map(|c| {
        translation_map.iter()
            .find_map(|(key, val)| if compare_string_char(val, c) {
                Some(key)
            } else {
                None
            }).unwrap()
    })
        .map(|x| x.to_string())
        .collect();

    values.join("").parse().expect("Unexpected input")
}

fn compare_string_char(a: &str, b: &str) -> bool {
    a.chars().into_iter().all(|ca| b.chars().any(|ba| ba == ca)) && b.chars().into_iter().all(|ca| a.chars().any(|ba| ba == ca))
}

fn diff_string(a: &str, b: &str) -> Vec<char> {
    a.chars().filter(|x| !b.chars().any(|y| y == *x)).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_8::{diff_string, solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(26, result)
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(61229, result)
    }

    #[test]
    fn should_diff_string() {
        let result = diff_string(&"gbe".to_string(), &"bg".to_string());
        assert_eq!(vec!['e'], result)
    }
}
