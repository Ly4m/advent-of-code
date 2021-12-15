use std::fs;

fn parse_input() -> Vec<((usize, usize), (usize, usize))> {
    fs::read_to_string("inputs/day_5.in").unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split("->");
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|x: (&str, &str)| -> ((usize, usize), (usize, usize)) { (parse_position(x.0), parse_position(x.1)) })
        .collect()
}

fn parse_position(position: &str) -> (usize, usize) {
    let mut split = position.trim().split(",");
    (split.next().map(|x| x.parse::<usize>()).unwrap().unwrap(), split.next().map(|x| x.parse::<usize>()).unwrap().unwrap())
}

fn mark_positions(istr: ((usize, usize), (usize, usize)), positions: &Vec<Vec<usize>>, with_diagonal: bool) -> Vec<Vec<usize>> {
    let mut new_positions = positions.clone();

    if istr.0.0 == istr.1.0 {
        if istr.0.1 > istr.1.1 {
            for x in istr.1.1..=istr.0.1 {
                new_positions[x][istr.0.0] += 1;
            }
        } else {
            for x in istr.0.1..=istr.1.1 {
                new_positions[x][istr.0.0] += 1;
            }
        }
    } else if istr.0.1 == istr.1.1 {
        if istr.0.0 > istr.1.0 {
            for x in istr.1.0..=istr.0.0 {
                new_positions[istr.0.1][x] += 1;
            }
        } else {
            for x in istr.0.0..=istr.1.0 {
                new_positions[istr.0.1][x] += 1;
            }
        }
    }
    else if istr.0.0 < istr.1.0 && istr.0.1 < istr.1.1 && with_diagonal {
        let mut x = istr.0.1;
        for y in istr.0.0..=istr.1.0 {
            new_positions[x][y] += 1;
            x += 1;
        }
    } else if istr.0.0 < istr.1.0 && istr.0.1 > istr.1.1 && with_diagonal {
        let mut x = istr.0.1;
        for y in istr.0.0..=istr.1.0 {
            new_positions[x][y] += 1;
            if x > 0 { x -= 1 };
        }
    } else if istr.0.0 > istr.1.0 && istr.0.1 > istr.1.1 && with_diagonal {
        let mut x = istr.1.1;
        for y in istr.1.0..=istr.0.0 {
            new_positions[x][y] += 1;
            x += 1;
        }
    } else if istr.0.0 > istr.1.0 && istr.0.1 < istr.1.1 && with_diagonal {
        let mut x = istr.1.1;
        for y in istr.1.0..=istr.0.0 {
            new_positions[x][y] += 1;
            if x > 0 { x -= 1 };
        }
    }

    new_positions
}

fn count_dangerous_positions(position: &Vec<Vec<usize>>) -> usize {
    let mut count = 0_usize;

    for y in position {
        for x in y {
            if *x > 1 { count += 1 }
        }
    };

    count
}

pub fn solve_part_1() -> usize {
    let instructions = parse_input();

    let mut positions: Vec<Vec<usize>> = vec![vec![0_usize; 1000]; 1000];

    for x in instructions {
        positions = mark_positions(x, &positions, false);
    }

    count_dangerous_positions(&positions)
}

pub fn solve_part_2() -> usize {
    let instructions = parse_input();

    let mut positions: Vec<Vec<usize>> = vec![vec![0_usize; 1000]; 1000];

    for x in instructions {
        positions = mark_positions(x, &positions, true);
    }

    count_dangerous_positions(&positions)
}

#[cfg(test)]
mod tests {
    use crate::day_5::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        let result = solve_part_1();
        assert_eq!(5124, result)
    }

    #[test]
    fn part_2() {
        let result = solve_part_2();
        assert_eq!(0, result)
    }
}
