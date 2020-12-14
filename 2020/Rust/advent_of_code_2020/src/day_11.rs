use std::fs;

fn parse_input() -> Vec<String> {
    fs::read_to_string("inputs/day_11.in").expect("Missing input file")
        .lines().map(String::from).collect()
}

pub fn solve_part_1() -> usize {
    let lines = parse_input();
    let width = lines.get(0).map(|l| l.len()).unwrap_or(0);

    let mut array = vec![vec!['.'; width]; width];

    for (i, line) in lines.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            array[i][y] = char;
        }
    }

    let mut current_array = array.clone();

    loop {

        let (new_array, is_modified) = next_array(&current_array);
        current_array = new_array;

        if !is_modified { break; }
    }

    count_occupied_seat(&current_array)
}

fn next_array(array: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut next_array = array.clone();
    let mut is_modified = false;

    for (x, row) in array.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {

            if *char == '.' { continue; }

            let mut counter = 0;

            if x > 0 {
                if y > 0 && array[x - 1][y - 1] == '#' { counter += 1 }
                if array[x - 1][y] == '#' { counter += 1 }
                if y < array.len() - 1 && array[x - 1][y + 1] == '#' { counter += 1 }
            }

            if x < array.len() -1 {
                if y > 0 && array[x + 1][y - 1] == '#' { counter += 1 }
                if array[x + 1][y] == '#' { counter += 1 }
                if y < array.len() - 1 && array[x + 1][y + 1] == '#' { counter += 1 }
            }

            if y > 0 && array[x][y - 1] == '#' { counter += 1 }
            if y < array.len() - 1 && array[x][y + 1] == '#' { counter += 1 }


            if *char == 'L' && counter == 0 {
                next_array[x][y] = '#';
                is_modified = true;
            } else if *char == '#' && counter >= 4 {
                next_array[x][y] = 'L';
                is_modified = true;
            }
        }
    }

    (next_array, is_modified)
}

fn print_array(array: &Vec<Vec<char>>) {
    for line in array {
        println!("{:?}", line)
    }
}

fn count_occupied_seat(array: &Vec<Vec<char>>) -> usize {
    array.iter().flatten().filter(|x| *x == &'#').count()
}

pub fn solve_part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_11::solve_part_1;

    #[test]
    fn test() {
        assert_eq!(0, solve_part_1())
    }
}
