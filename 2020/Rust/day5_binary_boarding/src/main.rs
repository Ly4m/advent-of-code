use std::fs;

fn main() {
    let lines = fs::read_to_string("src/boarding_pass.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let highest_seat = find_highest_seat_id(&lines);
    println!("The highest seat ID {}", highest_seat);

    let my_seat = find_missing_seat(&lines);
    println!("My seat : {}", my_seat)
}

fn decode_boarding_pass(pass: &str) -> (i32, i32) {
    let rows = &pass[0..7].replace('F', "0").replace("B", "1");
    let columns = &pass[7..10].replace('L', "0").replace("R", "1");

    let seat_row = i32::from_str_radix(rows, 2).unwrap();
    let seat_column = i32::from_str_radix(columns, 2).unwrap();

    (seat_row, seat_column)
}

fn compute_seat_id(seat_row: i32, seat_column: i32) -> i32 {
    (seat_row * 8) + seat_column
}

fn find_highest_seat_id(passes: &Vec<String>) -> i32 {
    passes.iter().map(|s| s.as_str())
        .map(|s| decode_boarding_pass(s))
        .map(|s| compute_seat_id(s.0, s.1))
        .max().unwrap()
}

fn find_missing_seat(passes: &Vec<String>) -> i32 {
    let mut all_seats = passes.iter().map(|s| s.as_str())
        .map(|s| decode_boarding_pass(s))
        .filter(|(row, column)| *row > 0 && *row < 127) // exclude first and last row
        .map(|s| compute_seat_id(s.0, s.1))
        .collect::<Vec<i32>>();

    all_seats.sort();

    *all_seats.iter()
        .enumerate()
        .find(|(index, seat)|  (*seat +1) != all_seats[index +1] )
        .unwrap().1 + 1
}


#[cfg(test)]
mod tests {
    use crate::compute_seat_id;
    use crate::decode_boarding_pass;

    #[test]
    fn should_decode_simple_boarding_pass() {
        let decoded_pass = decode_boarding_pass("FBFBBFFRLR");
        assert_eq!(357, compute_seat_id(decoded_pass.0, decoded_pass.1));
    }

    #[test]
    fn should_decode_simple_boarding_pass2() {
        let decoded_pass = decode_boarding_pass(("BFFFBBFRRR"));
        assert_eq!(567, (compute_seat_id(decoded_pass.0, decoded_pass.1)))
    }

    #[test]
    fn should_decode_simple_boarding_pass3() {
        let decoded_pass = decode_boarding_pass(("FFFBBBFRRR"));
        assert_eq!(119, compute_seat_id(decoded_pass.0, decoded_pass.1))
    }

    #[test]
    fn should_decode_simple_boarding_pass4() {
        let decoded_pass = decode_boarding_pass(("BBFFBBFRLL"));
        assert_eq!(820, compute_seat_id(decoded_pass.0, decoded_pass.1))
    }
}
