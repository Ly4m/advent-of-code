use std::fs;

use colour::dark_green_ln;
use colour::red_ln;
use regex::Regex;

fn main() {
    let lines = fs::read_to_string("src/passports.in").unwrap().lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut valid_passport_count = 0;
    let mut current_passport: Vec<String> = vec![];

    for line in lines {
        let attributes = line.split_whitespace().map(String::from).collect::<Vec<String>>();

        current_passport.extend(attributes.clone());

        // empty Line - end of the passport
        if attributes.len() == 0 {
            let required_attr = if current_passport.iter().any(|s| s.starts_with("cid")) { 8 } else { 7 };

            println!("=================", );
            println!("PASSWORD ANALYSIS");

            if current_passport.len() == required_attr {
                if check_all_attributes(current_passport) {
                    valid_passport_count += 1;
                }
            } else {
                red_ln!("REFUSED : missing attribute(s)");
            }

            current_passport = vec![];
        }
    }

    println!("=================", );
    println!("Valid password count : {}", valid_passport_count);
    println!("=================", );
}

fn check_attribute(attr_name: &str, value: &str, function: impl Fn(&str) -> bool) -> bool {
    let is_valid = function(value);
    if is_valid { dark_green_ln!("{}: {}", attr_name, value) } else { red_ln!("{}: {}", attr_name, value) }
    is_valid
}

fn check_all_attributes(passport: Vec<String>) -> bool {

    // Attributes closures
    let check_byr = |x: &str| is_number_between(x, 1920, 2002);
    let check_iyr = |x: &str| is_number_between(x, 2010, 2020);
    let check_eyr = |x: &str| is_number_between(x, 2020, 2030);
    let check_hgt = |x: &str| -> bool {
        return match x.ends_with("in") {
            true => is_number_between(x, 59, 76),
            false => is_number_between(x, 150, 193),
        }
    };
    let hexa_code = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    let check_hcl = |x: &str| hexa_code.is_match(x);
    let check_ecl = |x: &str| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|y| *y == x);
    let pid = Regex::new(r"^[0-9]{9}$").unwrap();
    let check_pid = |x: &str| pid.is_match(x);

    if passport.iter().all(|attr| {
        let key = &attr[0..3];
        let value = &attr[4..];

        match key {
            "byr" => check_attribute("byr", value, check_byr),
            "iyr" => check_attribute("iyr", value, check_iyr),
            "eyr" => check_attribute("eyr", value, check_eyr),
            "hgt" => check_attribute("hgt", value, check_hgt),
            "hcl" => check_attribute("hcl", value, check_hcl),
            "ecl" => check_attribute("ecl", value, check_ecl),
            "pid" => check_attribute("pid", value, check_pid),
            _ => true // ignore unknown key
        }
    }) {
        dark_green_ln!("ACCEPTED");
        true
    } else {
        red_ln!("REFUSED : invalid attribute(s)");
        false
    }
}

fn is_number_between(string: &str, low: i32, high: i32) -> bool {
    let number = string.replace("cm", "").replace("in", "").parse::<i32>();

    match &number {
        Err(_) => false,
        Ok(_) => {
            let n = number.unwrap();
            n >= low && n <= high
        },
    }
}


