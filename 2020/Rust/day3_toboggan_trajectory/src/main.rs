use std::fs;

fn main() {
    let lines = fs::read_to_string("src/slope.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let width = lines.get(0).unwrap().len();

    let mut count = count_trees(&lines, width, 1, 1);
    count *= count_trees(&lines, width, 1, 3);
    count *= count_trees(&lines, width, 1, 5);
    count *= count_trees(&lines, width, 1, 7);
    count *= count_trees(&lines, width, 2, 1);

    println!("{}", count);
}

fn count_trees(lines: &Vec<String>, width: usize, down_step:usize, right_step: usize) -> usize {

    let mut position  = 0;

    return lines
        .iter()
        .skip(down_step)
        .step_by(down_step)
        .filter(|l| {
            position += right_step; // move on x Axis

            if position >= width { position -= width; } // prevent out of bound

            l.chars().nth(position).unwrap() == '#' // if tree
        }).count();
}
