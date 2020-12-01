use std::fs;
use std::ops::Mul;

fn main() {
    let report: Vec<usize> = fs::read_to_string("src/report.in")
        .unwrap()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let result1 = repair_report(&report, 2020);
    println!("result for part 1 is {}", result1);

    let result = repair_report_2(&report, 2020);
    println!("result for part 2 is {}", result);
}

fn repair_report(report: &Vec<usize>, year: usize) -> usize {
    for (index, value) in report.iter().enumerate() {
        for value2 in report[index..report.len()].iter() {
            if value + value2 == year {
                return value.mul(value2);
            }
        }
    }
    return 0;
}

fn repair_report_2(report: &Vec<usize>, year: usize) -> usize {
    for (index, value) in report.iter().enumerate() {
        for (index2, value2) in report[index..report.len()].iter().enumerate() {
            // no need to iterate a third time if
            if value + value2 < year {
                for value3 in report[index2..report.len()].iter() {
                    if value + value2 + value3 == year {
                        return value.mul(value2).mul(value3);
                    }
                }
            }
        }
    }
    return -1;  // should not happen
}

#[cfg(test)]
mod tests {
    use crate::repair_report;
    use crate::repair_report_2;

    #[test]
    fn simple_repair_report() {
        let report: Vec<usize> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, repair_report(&report, 2020))
    }

     #[test]
    fn simple_repair_report_2() {
        let report: Vec<usize> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, repair_report_2(&report, 2020))
    }


}
