use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::read_to_string;
use std::iter::repeat;

fn parse_input(test_mode: bool, part: i8) -> String {
    let path = if test_mode {
        format!("inputs_test/day_9_{}.in", part)
    } else {
        "inputs/day_9.in".to_owned()
    };

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    read_to_string(reader).unwrap().trim().to_owned()
}

fn parse_disk_map(input: String) -> Vec<isize> {
    let mut output = Vec::new();

    let chars: Vec<char> = input.chars().collect();

    for (file_id, i) in (0..chars.len()).step_by(2).enumerate() {
        let file_size = chars[i].to_digit(10).unwrap() as usize;

        for _ in 0..file_size {
            output.push(file_id as isize);
        }

        if (i + 1) < chars.len() {
            let free_size = chars[i + 1].to_digit(10).unwrap() as usize;

            for _ in 0..free_size {
                output.push(-1);
            }
        }
    }

    output
}

fn defrag(disk_map: Vec<isize>) -> Vec<isize> {
    let mut fragmented_disk = VecDeque::from(disk_map);
    let mut defragmented_disk = Vec::new();

    loop {
        if let Some(current) = fragmented_disk.pop_front() {
            if current >= 0 {
                defragmented_disk.push(current);
                continue;
            } else {
                while let Some(tail) = fragmented_disk.pop_back() {
                    if tail < 0 {
                        continue;
                    }
                    if tail >= 0 {
                        defragmented_disk.push(tail);
                        break;
                    }
                }
            }
        }

        if fragmented_disk.is_empty() {
            break;
        }
    }

    defragmented_disk
}

fn calculate_checksum(compacted: &[isize]) -> usize {
    compacted
        .iter()
        .enumerate()
        .filter(|(_, &value)| value > 0)
        .map(|(pos, &id)| pos * id as usize)
        .sum()
}

pub fn solve_part_1(test_mode: bool) -> usize {
    let input = parse_input(test_mode, 1);
    let disk_map = parse_disk_map(input);
    let defragmented_disk = defrag(disk_map);
    calculate_checksum(&defragmented_disk)
}

pub fn solve_part_2(test_mode: bool) -> usize {
    let input = parse_input(test_mode, 1);
    let chars: Vec<_> = input.trim().chars().collect();

    let mut files = Vec::with_capacity(chars.len() / 2);
    let mut free = Vec::with_capacity(chars.len() / 2);

    for (i, &c) in chars.iter().enumerate() {
        let num = c.to_digit(10).unwrap() as i32;
        if i % 2 == 0 {
            files.push(num);
        } else {
            free.push(num);
        }
    }

    // Cheeky push of an empty free space so files and free are the same size
    free.push(0);

    let mut files = files.clone();
    let mut free = free.clone();
    let mut moved_files: Vec<Vec<(i32, i32)>> = vec![Vec::new(); files.len()];

    // Move files last to first
    let mut i = files.len() - 1;
    while i > 0 {
        // Attempt to find a free segment left to right that can fit this entire file
        let mut j = 0;
        while j < i {
            if free[j] >= files[i] && files[i] > 0 {
                // Reduce the free segment by the size of the moved file
                free[j] -= files[i];

                // Record the move
                moved_files[j].push((i as i32, files[i]));

                // The original file segment is now empty and expands the free space in front of it
                free[i - 1] += files[i];
                files[i] = 0;
                break;
            }
            j += 1;
        }
        i -= 1;
    }

    // Reconstruct the disk layout
    let mut disk = Vec::new();
    for (i, &fcount) in files.iter().enumerate() {
        // Unmoved file blocks
        disk.extend(repeat(i as i32).take(fcount as usize));

        // Moved files in this segment
        for &(idx, count) in &moved_files[i] {
            disk.extend(repeat(idx).take(count as usize));
        }

        // Remaining free space
        disk.extend(repeat(0).take(free[i] as usize));
    }

    // Compute the checksum
    disk.iter()
        .enumerate()
        .map(|(pos, &file_id)| pos * file_id as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_9::{solve_part_1, solve_part_2};
    #[test]
    fn part_1() {
        let result = solve_part_1(true);
        assert_eq!(result, 1928);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(true);
        assert_eq!(result, 2858);
    }
}
