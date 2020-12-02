use std::fs;

fn main() {
   let value =  fs::read_to_string("src/passwords.in")
        .unwrap()
        .lines()
        .map(String::from)
        .filter(|s| is_password_compliant(s))
        .count();

    println!("{}", value);
}

fn is_password_compliant(line: &str) -> bool {
    let splits: Vec<String> = line.split_whitespace()
        .map(String::from)
        .collect();

    let mut p1 = splits.get(0)?.split("-");
    let min = p1.next()?.parse::<usize>()?;
    let max = p1.next()?.parse::<usize>()?;

    let char = splits.get(1)?.chars().next()?;
    let pwd = splits.get(2)?;

    let count = pwd.chars().filter(|c| *c == char).count();

    return min <= count && max >= count;
}

#[cfg(test)]
mod tests {
    use crate::is_password_compliant;

    #[test]
    fn simple_test() {
        assert_eq!(true, is_password_compliant("1-3 a: abcde"))
    }

    #[test]
    fn simple_test2() {
        assert_eq!(false, is_password_compliant("1-3 b: cdefg"))
    }

    #[test]
    fn simple_test3() {
        assert_eq!(true, is_password_compliant("2-9 c: ccccccccc"))
    }
}
