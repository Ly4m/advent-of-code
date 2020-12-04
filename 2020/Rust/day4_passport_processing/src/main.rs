use std::fs;

fn main() {
    let lines = fs::read_to_string("src/passports.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut valid_passport_count = 0;

    let mut current_passport: Vec<String> = vec![];

    for line in lines {
        let attributes = line.split_whitespace().map(String::from).collect::<Vec<String>>();

        current_passport.extend(attributes.clone());

        if attributes.len() == 0 {

            println!("{:?}", current_passport);

            if current_passport.iter().any(|s| s.starts_with("cid")) {
                println!("{:?}", current_passport);
                if current_passport.len() == 8 {
                    valid_passport_count += 1;
                }
            } else {
                if current_passport.len() == 7 {
                    valid_passport_count += 1;
                }
            }
            current_passport = vec![];
        }
    }

    println!("{}", valid_passport_count);
}
