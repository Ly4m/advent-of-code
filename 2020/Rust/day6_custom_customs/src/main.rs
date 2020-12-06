use std::fs;
use std::collections::HashSet;

fn main() {
    let lines = fs::read_to_string("src/declaration_form.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut current_group = HashSet::new();
    let mut total_count = 0;

    for line in lines {
        for question in line.chars() {
            current_group.insert(question.to_string());
        }

        if line.is_empty() {
            total_count += current_group.len();
            current_group.drain();
            continue
        }
    }

    println!("{}", total_count);

}
