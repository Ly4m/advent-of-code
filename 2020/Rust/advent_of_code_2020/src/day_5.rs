use std::fs;

pub fn solve_part_1() -> usize {
    let lines = parse_input();
    find_highest_seat_id(&lines)
}

pub fn solve_part_2() -> usize {
    let lines = parse_input();
    find_missing_seat(&lines)
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("inputs/day_5.in").unwrap().lines()
        .map(String::from)
        .collect()
}

fn decode_boarding_pass(pass: &str) -> (usize, usize) {
    let rows = &pass[0..7].replace('F', "0").replace("B", "1");
    let columns = &pass[7..10].replace('L', "0").replace("R", "1");

    let seat_row = usize::from_str_radix(rows, 2).unwrap();
    let seat_column = usize::from_str_radix(columns, 2).unwrap();

    (seat_row, seat_column)
}

fn compute_seat_id(seat_row: usize, seat_column: usize) -> usize {
    (seat_row * 8) + seat_column
}

fn find_highest_seat_id(passes: &[String]) -> usize {
    passes.iter().map(|s| s.as_str())
        .map(|s| decode_boarding_pass(s))
        .map(|s| compute_seat_id(s.0, s.1))
        .max().unwrap() as usize
}

fn find_missing_seat(passes: &[String]) -> usize {
    let mut all_seats: Vec<usize> = passes.iter().map(|s| s.as_str())
        .map(|s| decode_boarding_pass(s))
        .filter(|(row, _column)| *row > 0 && *row < 127) // exclude first and last row
        .map(|s| compute_seat_id(s.0, s.1))
        .collect();

    all_seats.sort();

    *all_seats.iter()
        .enumerate()
        .find(|(index, seat)| (*seat + 1) != all_seats[index + 1])
        .unwrap().1 + 1
}


#[cfg(test)]
mod tests {
    use crate::day_5::{decode_boarding_pass, compute_seat_id};

    #[test]
    fn should_decode_simple_boarding_pass() {
        let decoded_pass = decode_boarding_pass("FBFBBFFRLR");
        assert_eq!(357, compute_seat_id(decoded_pass.0, decoded_pass.1));
    }

    #[test]
    fn should_decode_simple_boarding_pass2() {
        let decoded_pass = decode_boarding_pass("BFFFBBFRRR");
        assert_eq!(567, (compute_seat_id(decoded_pass.0, decoded_pass.1)))
    }

    #[test]
    fn should_decode_simple_boarding_pass3() {
        let decoded_pass = decode_boarding_pass("FFFBBBFRRR");
        assert_eq!(119, compute_seat_id(decoded_pass.0, decoded_pass.1))
    }

    #[test]
    fn should_decode_simple_boarding_pass4() {
        let decoded_pass = decode_boarding_pass("BBFFBBFRLL");
        assert_eq!(820, compute_seat_id(decoded_pass.0, decoded_pass.1))
    }
}
