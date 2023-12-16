// Day 10: Pipe Maze

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_tuple(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn p1(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let pipes: HashMap<char, Vec<Direction>> = [
        ('|', vec![Direction::Up, Direction::Down]),
        ('-', vec![Direction::Left, Direction::Right]),
        ('J', vec![Direction::Left, Direction::Up]),
        ('L', vec![Direction::Right, Direction::Up]),
        ('F', vec![Direction::Right, Direction::Down]),
        ('7', vec![Direction::Left, Direction::Down]),
        (
            'S',
            vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let s_pos = matrix
        .iter()
        .enumerate()
        .find_map(|(row, row_vec)| row_vec.iter().position(|&c| c == 'S').map(|col| (row, col)))
        .unwrap();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    seen.insert(s_pos);
    let mut count = 1;

    fn dfs_recursive(
        matrix: &Vec<Vec<char>>,
        pipes: &HashMap<char, Vec<Direction>>,
        seen: &mut HashSet<(usize, usize)>,
        count: &mut usize,
        current_node: (usize, usize),
    ) {
        if let Some(pipe_directions) = pipes.get(&matrix[current_node.0][current_node.1]) {
            for &direction in pipe_directions {
                let next_node = (
                    (current_node.0 as isize + direction.to_tuple().0) as usize,
                    (current_node.1 as isize + direction.to_tuple().1) as usize,
                );

                if next_node.0 >= matrix.len() || next_node.1 >= matrix[0].len() {
                    continue;
                }

                if pipes
                    .get(&matrix[next_node.0 as usize][next_node.1 as usize])
                    .is_none()
                {
                    continue;
                }

                if !pipes
                    .get(&matrix[next_node.0][next_node.1])
                    .unwrap()
                    .contains(&direction.opposite())
                {
                    continue;
                }

                if seen.insert(next_node) {
                    println!(
                        "New node from {:?} at {:?} to {:?} at {:?}, count: {:?}",
                        current_node,
                        matrix[current_node.0][current_node.1],
                        next_node,
                        matrix[next_node.0][next_node.1],
                        count
                    );
                    println!("Recurse!");
                    *count += 1;
                    dfs_recursive(matrix, pipes, seen, count, next_node);
                }
            }
        }
    }

    dfs_recursive(&matrix, &pipes, &mut seen, &mut count, s_pos);
    println!("Count: {:?}", count);
    count / 2
}

fn count_points_inside_loop(loop_chars: &[(usize, usize, char)]) -> usize {
    let sorted_loop: Vec<Vec<_>> = loop_chars
        .iter()
        .sorted_by_key(|&(row, _, _)| row)
        .group_by(|&(row, _, _)| row)
        .into_iter()
        .map(|(_, group)| {
            group
                .sorted_by_key(|&(_, col, _)| col)
                .cloned()
                .collect::<Vec<_>>()
        })
        .collect();

    let starter: Vec<char> = vec!['|', '7', 'J', 'S'];
    let ender: Vec<char> = vec!['|', 'L', 'F', 'S'];

    let result = sorted_loop.iter().fold(0, |acc, row| {
        row.iter()
            .filter(|(_, _, c)| starter.contains(c) || ender.contains(c))
            .tuples()
            .filter_map(|(pair1, pair2)| {
                if !starter.contains(&pair1.2) || !ender.contains(&pair2.2) {
                    return None;
                }

                Some(pair2.1 - pair1.1 - 1)
            })
            .sum::<usize>()
            + acc
    });

    result
}

fn p2(input: &str) -> usize {
    // TODO: find area inside loop

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let pipes: HashMap<char, Vec<Direction>> = [
        ('|', vec![Direction::Up, Direction::Down]),
        ('-', vec![Direction::Left, Direction::Right]),
        ('J', vec![Direction::Left, Direction::Up]),
        ('L', vec![Direction::Right, Direction::Up]),
        ('F', vec![Direction::Right, Direction::Down]),
        ('7', vec![Direction::Left, Direction::Down]),
        (
            'S',
            vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let s_pos = matrix
        .iter()
        .enumerate()
        .find_map(|(row, row_vec)| row_vec.iter().position(|&c| c == 'S').map(|col| (row, col)))
        .unwrap();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut loop_list: Vec<(usize, usize, char)> = Vec::new();

    loop_list.push((s_pos.0, s_pos.1, matrix[s_pos.0][s_pos.1]));
    seen.insert(s_pos);

    fn dfs_recursive(
        matrix: &Vec<Vec<char>>,
        pipes: &HashMap<char, Vec<Direction>>,
        seen: &mut HashSet<(usize, usize)>,
        loop_list: &mut Vec<(usize, usize, char)>,
        current_node: (usize, usize),
    ) {
        if let Some(pipe_directions) = pipes.get(&matrix[current_node.0][current_node.1]) {
            for &direction in pipe_directions {
                let next_node = (
                    (current_node.0 as isize + direction.to_tuple().0) as usize,
                    (current_node.1 as isize + direction.to_tuple().1) as usize,
                );

                if next_node.0 >= matrix.len() || next_node.1 >= matrix[0].len() {
                    continue;
                }

                if pipes
                    .get(&matrix[next_node.0 as usize][next_node.1 as usize])
                    .is_none()
                {
                    continue;
                }

                if !pipes
                    .get(&matrix[next_node.0][next_node.1])
                    .unwrap()
                    .contains(&direction.opposite())
                {
                    continue;
                }

                if seen.insert(next_node) {
                    loop_list.insert(
                        loop_list.len() - 1,
                        (next_node.0, next_node.1, matrix[next_node.0][next_node.1]),
                    );
                    dfs_recursive(matrix, pipes, seen, loop_list, next_node);
                }
            }
        }
    }

    dfs_recursive(&matrix, &pipes, &mut seen, &mut loop_list, s_pos);
    let num_points_inside_loop = count_points_inside_loop(&loop_list);
    num_points_inside_loop as usize
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
        assert_eq!(p1(input), 8);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 10);
    }
}
