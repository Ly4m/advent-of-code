use std::fs;

fn main() {
    let lines = fs::read_to_string("src/slope.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let length = lines.get(0).unwrap().len();
    let count = count_trees(lines, length);

    println!("{}", count);
}

fn count_trees(lines: Vec<String>, length: usize) -> usize {

    let mut position  = 0;

    return lines
        .iter()
        .skip(1)
        .filter(|l| {

            position += 3; // move on x Axis

            if position >= length {  // if going out of bound
                position -= length;
            }

            l.chars().nth(position).unwrap() == '#' // if tree
        }).count();
}
