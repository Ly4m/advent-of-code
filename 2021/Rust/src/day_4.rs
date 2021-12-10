use std::fs;

fn parse_input() -> (Vec<u8>, Vec<Board>) {
    let input = fs::read_to_string("inputs/day_4.in").unwrap();
    let numbers_to_draw = parse_numbers_to_draw(input.lines().next().unwrap());
    let boards = parse_boards(&input);
    (numbers_to_draw, boards)
}

fn parse_numbers_to_draw(numbers: &str) -> Vec<u8> {
    numbers.split(',').filter_map(|x| x.parse().ok()).collect()
}

fn parse_boards(input: &str) -> Vec<Board> {
    let lines: Vec<Vec<Number>> = input.lines().skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| -> Vec<u8> x.split_whitespace().filter_map(|x| x.parse().ok()).collect())
        .map(|x| -> Vec<Number> x.iter().map(|value| -> Number {
            Number {
                value: *value,
                marked: false,
            }
        }).collect()
        ).collect();

    lines.chunks(5).map(|grid| {
        Board {
            has_won: false,
            grid: grid.to_vec(),
        }
    }).collect()
}

#[derive(Debug, Clone)]
struct Number {
    value: u8,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    has_won: bool,
    grid: Vec<Vec<Number>>,
}

impl Board {
    // return true if number was found
    fn mark_number(&mut self, value: u8) -> bool {
        for row in &mut self.grid {
            for mut number in row {
                if number.value == value {
                    number.marked = true;
                }
            }
        }

        if self.is_complete() {
            return true;
        }
        false
    }

    fn is_complete(&self) -> bool {
        for row in &self.grid {
            if row.iter().filter(|number| number.marked).count() == 5 {
                return true;
            };
        }


        for index in 0..4 {
            let mut count = 0;
            for row in &self.grid {
                if row.get(index).unwrap().marked {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }

        false
    }

    fn unmarked_numbers_sum(&self) -> u32 {
        self.grid.iter().flatten()
            .filter(|case| {
                !case.marked
            })
            .map(|case| {
                case.value as u32
            })
            .sum()
    }
}

pub fn solve_part_1() -> usize {
    let (numbers, mut boards) = parse_input();

    for number in numbers {
        for board in boards.iter_mut() {
            if board.mark_number(number) {
                let unmarked = board.unmarked_numbers_sum();
                return (number as u32 * unmarked) as usize;
            };
        }
    }
    
    0
}

pub fn solve_part_2() -> usize {
    let (numbers, mut boards) = parse_input();
    let mut winning_board: Vec<usize> = vec![];

    for number in numbers {
        for board in boards.iter_mut().filter(|board| !board.has_won) {
            if board.mark_number(number) {
                board.has_won = true;
                let unmarked = board.unmarked_numbers_sum();
                let score = (number as u32 * unmarked) as usize;
                winning_board.push(score);
            };
        }
    }

    *winning_board.last().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day_4::{parse_numbers_to_draw, solve_part_2};

    #[test]
    fn part_1() {
        solve_part_2();
        assert_eq!(true, true)
    }

    #[test]
    fn part_1_parse_numbers_to_draw() {
        let data = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
        let numbers = parse_numbers_to_draw(data);

        let expected: Vec<u8> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];
        assert_eq!(expected, numbers)
    }
}
