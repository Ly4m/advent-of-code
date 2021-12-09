use std::fs;

fn parse_input() -> Vec<String> {
    fs::read_to_string("inputs/day_3.in")
        .unwrap()
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn solve_part_1() -> usize {
    let inputs: Vec<String> = parse_input();
    check_power_consumption(inputs)
}

pub fn solve_part_2() -> usize {
    let inputs: Vec<String> = parse_input();

    let oxygen = process_rating(&inputs, 0, calculate_gamma_with_equality_override, '1');
    let co2 = process_rating(&inputs, 0, calculate_epsilon_with_equality_override, '0');

    usize::from_str_radix(oxygen.as_str(), 2).unwrap() * usize::from_str_radix(co2.as_str(), 2).unwrap()
}

fn check_power_consumption(inputs: Vec<String>) -> usize {
    let gamma = calculate_gamma_with_equality_override(&inputs);
    let epsilon = reverse_bit_string(&gamma);

    usize::from_str_radix(gamma.as_str(), 2).unwrap() * usize::from_str_radix(epsilon.as_str(), 2).unwrap()
}

fn reverse_bit_string(bits: &String) -> String {
    bits.chars().map(|x| if x == '1' { "0" } else { "1" }).collect()
}

fn calculate_gamma_with_equality_override(inputs: &Vec<String>) -> String {
    let equality_override = "1".to_string();
    let vector_length = inputs.get(0).unwrap().len();
    let half_input_length = inputs.len() as f32 / 2f32;

    let bits_count = count_bits(inputs, vector_length);

    bits_count.iter().map(|x| {
        if x == &half_input_length { equality_override.clone() } else if x > &half_input_length { "1".to_string() } else { "0".to_string() }
    }).collect()
}

fn calculate_epsilon_with_equality_override(inputs: &Vec<String>) -> String {
    let equality_override = "0".to_string();
    let vector_length = inputs.get(0).unwrap().len();
    let half_input_length = inputs.len() as f32 / 2f32;

    let bits_count = count_bits(inputs, vector_length);

    bits_count.iter().map(|x| {
        if x == &half_input_length { equality_override.clone() } else if x < &half_input_length { "1".to_string() } else { "0".to_string() }
    }).collect()
}

fn count_bits(inputs: &Vec<String>, vector_length: usize) -> Vec<f32> {
    let inputs_as_u32: Vec<Vec<f32>> = inputs
        .iter()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as f32).collect())
        .collect();

    let mut aggregator: Vec<f32> = vec![0 as f32; vector_length];

    for line in inputs_as_u32 {
        for bit_idx in 0..vector_length {
            aggregator[bit_idx] += line[bit_idx]
        }
    }

    aggregator
}

fn process_rating(inputs: &Vec<String>, index: usize, f: fn(&Vec<String>) -> String, equality_override: char) -> String {
    if inputs.len() == 1 as usize {
        return inputs.get(0).unwrap().to_string();
    }

    if index == inputs.get(0).unwrap().len() - 1 {
        for line in inputs {
            if line.chars().nth(index).unwrap() == equality_override {
                return line.to_string();
            }
        }
    }

    let mask = f(inputs);

    let filtered_list: Vec<String> = inputs.iter().
        filter(|x|
            x.chars().nth(index).unwrap() == mask.chars().nth(index).unwrap()
        ).map(|x| x.to_string())
        .collect();

    process_rating(&filtered_list, index + 1, f, equality_override)
}

#[cfg(test)]
mod tests {
    use crate::day_3::{check_power_consumption, process_rating, calculate_gamma_with_equality_override, calculate_epsilon_with_equality_override};

    #[test]
    fn part_1() {
        let data = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ].iter().map(|x| x.to_string()).collect();

        let result = check_power_consumption(data);
        assert_eq!(198, result)
    }

    #[test]
    fn should_find_oxygen_rating() {
        let data = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ].iter().map(|x| x.to_string()).collect();

        let result = process_rating(&data, 0, calculate_gamma_with_equality_override, '1');
        assert_eq!("10111".to_string(), result);

        let result_epsilon = process_rating(&data, 0, calculate_epsilon_with_equality_override, '0');
        assert_eq!("01010".to_string(), result_epsilon);
    }
}
