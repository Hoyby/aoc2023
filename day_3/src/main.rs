// Day 3: Gear Ratios

use std::{
    collections::{HashMap, HashSet},
    usize,
};

enum Direction {
    Lt,
    Rt,
    Up,
    Dn,
    At,
}

impl Direction {
    fn delta(&self, i: usize, n: usize) -> Option<usize> {
        match self {
            Direction::Lt | Direction::Up => i.checked_sub(1),
            Direction::Rt | Direction::Dn => {
                if i < n - 1 {
                    Some(i + 1)
                } else {
                    None
                }
            }
            _ => Some(i),
        }
    }
}

fn p1(input: &str) -> i32 {
    let matrix = input
        .lines()
        .into_iter()
        .map(|s| s.bytes().collect())
        .collect::<Vec<Vec<_>>>();

    let (rows, columns) = (matrix.len(), matrix[0].len());

    let mut result = 0;
    let mut seen = HashSet::new();

    for row in 0..rows {
        for col in 0..columns {
            if matrix[row][col] != b'.' && (matrix[row][col] < b'0' || matrix[row][col] > b'9') {
                for (dir_row, dir_col) in [
                    (Direction::Up, Direction::Lt),
                    (Direction::Up, Direction::At),
                    (Direction::Up, Direction::Rt),
                    (Direction::At, Direction::Lt),
                    (Direction::At, Direction::At),
                    (Direction::At, Direction::Rt),
                    (Direction::Dn, Direction::Lt),
                    (Direction::Dn, Direction::At),
                    (Direction::Dn, Direction::Rt),
                ] {
                    if let (Some(new_row), Some(new_col)) =
                        (dir_row.delta(row, rows), dir_col.delta(col, columns))
                    {
                        if matrix[new_row][new_col].is_ascii_digit() {
                            let (number, digit_index_start, digit_index_end) = {
                                let mut index_start = new_col;
                                while index_start > 0
                                    && matrix[new_row][index_start - 1].is_ascii_digit()
                                {
                                    index_start -= 1;
                                }
                                let mut index_end = new_col;
                                while index_end < matrix[new_row].len()
                                    && matrix[new_row][index_end].is_ascii_digit()
                                {
                                    index_end += 1;
                                }
                                let number =
                                    std::str::from_utf8(&&matrix[new_row][index_start..index_end])
                                        .unwrap()
                                        .parse::<i32>()
                                        .unwrap();
                                (number, index_start, index_end)
                            };

                            if seen.insert((new_row, digit_index_start, digit_index_end)) {
                                result += number;
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn p2(input: &str) -> usize {
    let matrix = input
        .lines()
        .map(|s| s.bytes().collect())
        .collect::<Vec<Vec<_>>>();

    let (rows, columns) = (matrix.len(), matrix[0].len());

    let mut seen = HashSet::new();
    let mut adjacent_nums: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for row in 0..rows {
        for col in 0..columns {
            if matrix[row][col] == b'*' {
                for (dir_row, dir_col) in [
                    (Direction::Up, Direction::Lt),
                    (Direction::Up, Direction::At),
                    (Direction::Up, Direction::Rt),
                    (Direction::At, Direction::Lt),
                    (Direction::At, Direction::Rt),
                    (Direction::Dn, Direction::Lt),
                    (Direction::Dn, Direction::At),
                    (Direction::Dn, Direction::Rt),
                ] {
                    if let (Some(new_row), Some(new_col)) =
                        (dir_row.delta(row, rows), dir_col.delta(col, columns))
                    {
                        if matrix[new_row][new_col].is_ascii_digit() {
                            let (number, digit_index_start, digit_index_end) = {
                                let mut index_start = new_col;
                                while index_start > 0
                                    && matrix[new_row][index_start - 1].is_ascii_digit()
                                {
                                    index_start -= 1;
                                }
                                let mut index_end = new_col;
                                while index_end < matrix[new_row].len()
                                    && matrix[new_row][index_end].is_ascii_digit()
                                {
                                    index_end += 1;
                                }
                                let number =
                                    std::str::from_utf8(&&matrix[new_row][index_start..index_end])
                                        .unwrap()
                                        .parse::<usize>()
                                        .unwrap();
                                (number, index_start, index_end)
                            };

                            let key = (row, col);
                            let entry = adjacent_nums.entry(key).or_insert(vec![]);
                            if seen.insert((new_row, digit_index_start, digit_index_end)) {
                                entry.push(number);
                            }
                        }
                    }
                }
            }
        }
    }

    adjacent_nums
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(_, v)| v.iter().product::<usize>())
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = include_str!("../example_1.txt");
        assert_eq!(p1(input), 4361);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 467835);
    }
}
