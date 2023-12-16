// Day 16: The Floor Will Be Lava

use std::collections::{HashSet, VecDeque};

fn get_tile(map: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<char> {
    let (x, y) = pos;
    if x >= map[0].len() || y >= map.len() {
        return None;
    }
    Some(map[x][y])
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
}

fn get_next_tiles(
    map: &Vec<Vec<char>>,
    curr_pos_dir: ((usize, usize), Direction),
) -> Vec<((usize, usize), Direction)> {
    let curr_pos = curr_pos_dir.0;
    let curr_direction = curr_pos_dir.1;

    let next_directions: Vec<Direction> = match get_tile(map, curr_pos) {
        Some('/') => match curr_direction {
            Direction::Up => vec![Direction::Right],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
            Direction::Right => vec![Direction::Up],
        },
        Some('\\') => match curr_direction {
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
            Direction::Right => vec![Direction::Down],
        },
        Some('-') => match curr_direction {
            Direction::Up => vec![Direction::Left, Direction::Right],
            Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Left],
            Direction::Right => vec![Direction::Right],
        },
        Some('|') => match curr_direction {
            Direction::Up => vec![Direction::Up],
            Direction::Down => vec![Direction::Down],
            Direction::Left => vec![Direction::Up, Direction::Down],
            Direction::Right => vec![Direction::Up, Direction::Down],
        },
        Some('.') => match curr_direction {
            Direction::Up => vec![Direction::Up],
            Direction::Down => vec![Direction::Down],
            Direction::Left => vec![Direction::Left],
            Direction::Right => vec![Direction::Right],
        },
        _ => panic!("Invalid tile"),
    };

    next_directions
        .iter()
        .map(|direction| {
            let (x, y) = curr_pos;
            let (dx, dy) = direction.to_tuple();
            let next_pos = (x as isize + dx, y as isize + dy);
            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
            (next_pos, *direction)
        })
        .filter(|(pos, _)| get_tile(map, *pos).is_some())
        .collect()
}

fn p1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|map| map.chars().filter(|&c| c != ' ').collect::<Vec<char>>())
        .collect();

    let mut stack: Vec<((usize, usize), Direction)> = vec![((0, 0), Direction::Right)];
    let mut seen: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut color_map: Vec<Vec<char>> = vec![vec!['.'; map[0].len()]; map.len()];

    loop {
        if stack.is_empty() {
            break;
        }

        let curr_pos_dir = stack.pop().unwrap();
        if seen.contains(&curr_pos_dir) {
            continue;
        }
        let next_tiles = get_next_tiles(&map, curr_pos_dir);
        color_map[curr_pos_dir.0 .0][curr_pos_dir.0 .1] = '#';
        seen.insert(curr_pos_dir);
        stack.extend(next_tiles);
    }
    let unique_positions: HashSet<(usize, usize)> = seen.iter().map(|&(pos, _)| pos).collect();
    unique_positions.len()
}

fn p2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut max_seen_count = 0;
    let mut max_start_pos = (0, 0);
    let mut max_start_dir = Direction::Right;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if i == 0 || i == map.len() - 1 || j == 0 || j == map[0].len() - 1 {
                let start_pos = (i, j);
                let start_dir = if i == 0 {
                    Direction::Down
                } else if i == map.len() - 1 {
                    Direction::Up
                } else if j == 0 {
                    Direction::Right
                } else {
                    Direction::Left
                };

                let mut stack: VecDeque<((usize, usize), Direction)> = VecDeque::new();
                stack.push_back((start_pos, start_dir));

                let mut seen: HashSet<((usize, usize), Direction)> = HashSet::new();
                let mut color_map: Vec<Vec<char>> = vec![vec!['.'; map[0].len()]; map.len()];

                while let Some(curr_pos_dir) = stack.pop_back() {
                    if seen.contains(&curr_pos_dir) {
                        continue;
                    }

                    let next_tiles = get_next_tiles(&map, curr_pos_dir);
                    color_map[curr_pos_dir.0 .0][curr_pos_dir.0 .1] = '#';
                    seen.insert(curr_pos_dir);
                    stack.extend(next_tiles);
                }
                let unique_positions: HashSet<(usize, usize)> =
                    seen.iter().map(|&(pos, _)| pos).collect();

                if unique_positions.len() > max_seen_count {
                    max_seen_count = unique_positions.len();
                    max_start_pos = start_pos;
                    max_start_dir = start_dir;
                }
            }
        }
    }
    println!(
        "Max start pos: {:?}, Max start dir: {:?}",
        max_start_pos, max_start_dir
    );

    max_seen_count
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
        assert_eq!(p1(input), 46);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 51);
    }
}
