use std::fs;

fn main() {
    let lines = fs::read_to_string("src/boarding_pass.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let highest_seat = find_highest_seat_id(&lines);
    println!("{}", highest_seat)
}

fn decode_boarding_pass(pass: &str) -> i32 {

    let rows = &pass[0..7].replace('F',"0").replace("B", "1");
    let columns = &pass[7..10].replace('L',"0").replace("R", "1");

    let seat_row = i32::from_str_radix(rows, 2).unwrap();
    let seat_column = i32::from_str_radix(columns, 2).unwrap();

    (seat_row * 8) + seat_column
}

fn find_highest_seat_id(passes: &Vec<String>) -> i32 {
    passes.iter().map(|s| s.as_str())
        .map(|s| decode_boarding_pass(s))
        .max().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::decode_boarding_pass;

    #[test]
    fn should_decode_simple_boarding_pass() {
        assert_eq!(357, decode_boarding_pass("FBFBBFFRLR"))
    }

    #[test]
    fn should_decode_simple_boarding_pass2() {
        assert_eq!(567, decode_boarding_pass("BFFFBBFRRR"))
    }

    #[test]
    fn should_decode_simple_boarding_pass3() {
        assert_eq!(119, decode_boarding_pass("FFFBBBFRRR"))
    }

    #[test]
    fn should_decode_simple_boarding_pass4() {
        assert_eq!(820, decode_boarding_pass("BBFFBBFRLL"))
    }
}
