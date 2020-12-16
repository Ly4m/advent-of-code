use std::fs;

fn parse_input() -> Vec<Vec<char>> {
    let lines: Vec<String> = fs::read_to_string("inputs/day_11.in").expect("Missing input file")
        .lines().map(String::from).collect();

    let width = lines.get(0).map(|l| l.len()).unwrap_or(0);
    let mut array = vec![vec!['.'; width]; width];

    for (i, line) in lines.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            array[i][y] = char;
        }
    }

    array
}

pub fn solve_part_1() -> usize {
    let array = parse_input();
    let mut current_array = array;

    loop {
        let (new_array, is_modified) = next_array_part_1(&current_array);
        current_array = new_array;

        if !is_modified { break; }
    }

    count_occupied_seat(&current_array)
}

fn next_array_part_1(seats: &[Vec<char>]) -> (Vec<Vec<char>>, bool) {
    let mut next_array = seats.to_owned();
    let mut is_modified = false;

    for (x, row) in seats.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char == '.' { continue; }

            let mut counter = 0;

            if x > 0 {
                if y > 0 && seats[x - 1][y - 1] == '#' { counter += 1 }
                if seats[x - 1][y] == '#' { counter += 1 }
                if y < seats.len() - 1 && seats[x - 1][y + 1] == '#' { counter += 1 }
            }

            if x < seats.len() - 1 {
                if y > 0 && seats[x + 1][y - 1] == '#' { counter += 1 }
                if seats[x + 1][y] == '#' { counter += 1 }
                if y < seats.len() - 1 && seats[x + 1][y + 1] == '#' { counter += 1 }
            }

            if y > 0 && seats[x][y - 1] == '#' { counter += 1 }
            if y < seats.len() - 1 && seats[x][y + 1] == '#' { counter += 1 }


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
//
// fn print_array(array: &Vec<Vec<char>>) {
//     for line in array {
//         println!("{:?}", line)
//     }
// }

fn count_occupied_seat(array: &[Vec<char>]) -> usize {
    array.iter().flatten().filter(|x| *x == &'#').count()
}

pub fn solve_part_2() -> usize {
    let array = parse_input();
    let mut current_array = array;

    loop {
        let (new_array, is_modified) = next_array_part_2(&current_array);
        current_array = new_array;

        if !is_modified { break; }
    }

    count_occupied_seat(&current_array)
}

fn next_array_part_2(seats: &[Vec<char>]) -> (Vec<Vec<char>>, bool) {
    let mut next_array = seats.to_owned();
    let mut is_modified = false;

    for (x, row) in seats.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char == '.' { continue; }

            let mut counter = 0;

            counter += scan_in_direction(x as isize, y as isize + 1, seats, (0 as isize, 1 as isize));
            counter += scan_in_direction(x as isize + 1, y as isize + 1, seats, (1 as isize, 1 as isize));
            counter += scan_in_direction(x as isize + 1, y as isize, seats, (1 as isize, 0 as isize));
            counter += scan_in_direction(x as isize, y as isize - 1, seats, (0 as isize, -1 as isize));
            counter += scan_in_direction(x as isize + 1, y as isize - 1, seats, (1 as isize, -1 as isize));
            counter += scan_in_direction(x as isize - 1, y as isize + -1, seats, (-1 as isize, -1 as isize));
            counter += scan_in_direction(x as isize - 1, y as isize, seats, (-1 as isize, 0 as isize));
            counter += scan_in_direction(x as isize - 1, y as isize + 1, seats, (-1 as isize, 1 as isize));

            if *char == 'L' && counter == 0 {
                next_array[x][y] = '#';
                is_modified = true;
            } else if *char == '#' && counter >= 5 {
                next_array[x][y] = 'L';
                is_modified = true;
            }
        }
    }

    (next_array, is_modified)
}

fn scan_in_direction(x: isize, y: isize, seats: &[Vec<char>], direction: (isize, isize)) -> isize {
    let height = seats.len() as isize;

    if x < 0 || y < 0 || x == height || y == height {
        return 0;
    }

    match seats[x as usize][y as usize] {
        '#' => 1,
        'L' => 0,
        '.' => scan_in_direction(x + direction.0, y + direction.1, seats, direction),
        _ => panic!("HEIN ?!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::day_11::solve_part_1;

    #[test]
    fn test() {
        assert_eq!(0, solve_part_1())
    }
}
