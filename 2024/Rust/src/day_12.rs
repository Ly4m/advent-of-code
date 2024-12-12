use std::fs::File;
use std::io;
use std::io::BufRead;

fn parse_input(test_mode: bool, part: i8) -> Vec<Vec<char>> {
    let path = if test_mode {
        format!("inputs_test/day_12_{}.in", part)
    } else {
        "inputs/day_12.in".to_owned()
    };
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let garden = parse_input(test_mode, 1);
    let height = garden.len();
    let width = garden[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut total_cost = 0;

    for i in 0..height {
        for j in 0..width {
            if !visited[i][j] {
                // Found a new region
                let plant_type = garden[i][j];
                let (area, perimeter) =
                    explore_region(&garden, &mut visited, i, j, height, width, plant_type);
                total_cost += area * perimeter;
            }
        }
    }

    total_cost
}

fn explore_region(
    garden: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start_i: usize,
    start_j: usize,
    h: usize,
    w: usize,
    plant_type: char,
) -> (usize, usize) {
    let mut stack = vec![(start_i, start_j)];
    visited[start_i][start_j] = true;

    let mut cells = Vec::new();
    cells.push((start_i, start_j));

    while let Some((x, y)) = stack.pop() {
        // Explore neighbors
        for (nx, ny) in neighbors(x, y, h, w) {
            if !visited[nx][ny] && garden[nx][ny] == plant_type {
                visited[nx][ny] = true;
                stack.push((nx, ny));
                cells.push((nx, ny));
            }
        }
    }

    let area = cells.len();
    let perimeter = calc_perimeter(&cells, garden, h, w, plant_type);

    (area, perimeter)
}

fn explore_region_for_discount(
    garden: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start_i: usize,
    start_j: usize,
    h: usize,
    w: usize,
    plant_type: char,
) -> (usize, usize) {
    let mut stack = vec![(start_i, start_j)];
    visited[start_i][start_j] = true;

    let mut cells = Vec::new();
    cells.push((start_i, start_j));

    while let Some((x, y)) = stack.pop() {
        // Explore neighbors
        for (nx, ny) in neighbors(x, y, h, w) {
            if !visited[nx][ny] && garden[nx][ny] == plant_type {
                visited[nx][ny] = true;
                stack.push((nx, ny));
                cells.push((nx, ny));
            }
        }
    }

    let area = cells.len();
    let corner = count_corner(&cells, garden, h, w, plant_type);

    (area, corner)
}

fn count_corner(
    cells: &[(usize, usize)],
    garden: &[Vec<char>],
    h: usize,
    w: usize,
    plant_type: char,
) -> usize {
    let mut corners_count = 0;

    for &(x, y) in cells {
        let mut different_cells_around = 0;
        let mut found_sides: Vec<char> = vec![];
        let mut found_adjacent: Vec<char> = vec![];

        if x == 0 || garden[x - 1][y] != plant_type {
            different_cells_around += 1;
            found_sides.push('N');
        } else if x != 0 && garden[x - 1][y] == plant_type {
            found_adjacent.push('N');
        }
        if x == h - 1 || garden[x + 1][y] != plant_type {
            found_sides.push('S');
            different_cells_around += 1;
        } else if x != h - 1 && garden[x + 1][y] == plant_type {
            found_adjacent.push('S');
        }
        if y == 0 || garden[x][y - 1] != plant_type {
            found_sides.push('W');
            different_cells_around += 1;
        } else if y != 0 && garden[x][y - 1] == plant_type {
            found_adjacent.push('W');
        }
        if y == w - 1 || garden[x][y + 1] != plant_type {
            found_sides.push('E');
            different_cells_around += 1;
        } else if y != w - 1 && garden[x][y + 1] == plant_type{
            found_adjacent.push('E');
        }

        if different_cells_around == 2 {
            if !(found_sides.contains(&'N') && found_sides.contains(&'S')
                || found_sides.contains(&'W') && found_sides.contains(&'E'))
            {
                corners_count += 1;
            }
        } else if different_cells_around == 3 {
            corners_count += 2;
        } else if different_cells_around == 4 {
            corners_count += 4;
        }

        if different_cells_around <= 4 {
            if x != 0
                && y != w - 1
                && found_adjacent.contains(&'N')
                && found_adjacent.contains(&'E')
                && garden[x - 1][y + 1] != plant_type
            {
                corners_count += 1;
            }
            if x != 0
                && y != 0
                && found_adjacent.contains(&'N')
                && found_adjacent.contains(&'W')
                && garden[x - 1][y - 1] != plant_type
            {
                corners_count += 1;
            }
            if x != h - 1
                && y != w - 1
                && found_adjacent.contains(&'S')
                && found_adjacent.contains(&'E')
                && garden[x + 1][y + 1] != plant_type
            {
                corners_count += 1;
            }
            if x != h - 1
                && y != 0
                && found_adjacent.contains(&'S')
                && found_adjacent.contains(&'W')
                && garden[x + 1][y - 1] != plant_type
            {
                corners_count += 1;
            }
        }
    }

    corners_count
}

fn calc_perimeter(
    cells: &[(usize, usize)],
    garden: &[Vec<char>],
    h: usize,
    w: usize,
    plant_type: char,
) -> usize {
    let mut perimeter = 0;

    for &(x, y) in cells {
        if x == 0 || garden[x - 1][y] != plant_type {
            perimeter += 1;
        }
        if x == h - 1 || garden[x + 1][y] != plant_type {
            perimeter += 1;
        }
        if y == 0 || garden[x][y - 1] != plant_type {
            perimeter += 1;
        }
        if y == w - 1 || garden[x][y + 1] != plant_type {
            perimeter += 1;
        }
    }

    perimeter
}

fn neighbors(x: usize, y: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 {
        result.push((x - 1, y));
    }
    if x + 1 < h {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y + 1 < w {
        result.push((x, y + 1));
    }
    result
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let garden = parse_input(test_mode, 1);
    let height = garden.len();
    let width = garden[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut total_cost = 0;

    for i in 0..height {
        for j in 0..width {
            if !visited[i][j] {
                // Found a new region
                let plant_type = garden[i][j];
                let (area, perimeter) = explore_region_for_discount(
                    &garden,
                    &mut visited,
                    i,
                    j,
                    height,
                    width,
                    plant_type,
                );
                total_cost += area * perimeter;
            }
        }
    }

    total_cost
}

#[cfg(test)]
mod tests {
    use crate::day_12::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 1930);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 1206);
    }
}
