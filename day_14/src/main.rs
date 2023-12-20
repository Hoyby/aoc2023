// Day 14: Parabolic Reflector Dish

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn p1(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for col in 0..map[0].len() {
        for row in 0..map.len() {
            if map[row][col] == 'O' {
                let mut rolldown_row = row;
                while rolldown_row > 0 && map[rolldown_row - 1][col] == '.' {
                    map[rolldown_row - 1][col] = 'O';
                    map[rolldown_row][col] = '.';
                    rolldown_row -= 1;
                }
            }
        }
    }

    map.iter()
        .rev()
        .enumerate()
        .map(|(row, r)| r.iter().filter(|&&c| c == 'O').count() * (row + 1))
        .sum()
}

fn p2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn roll_rocks_memoized(
        mut map: Vec<Vec<char>>,
        total_iterations: usize,
        memoization_cache: &mut HashMap<Vec<Vec<char>>, (Vec<Vec<char>>, usize)>,
    ) -> Vec<Vec<char>> {
        let directions = vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];

        for iteration in 0..total_iterations - 1 {
            let mut map_clone = map.clone();

            for direction in &directions {
                if let Some(result) = memoization_cache.get(&(map.clone())) {
                    let result_iteration = result.1;
                    let result_map = result.0.clone();
                    let cycle_len = iteration - result_iteration;
                    let new_total = (total_iterations - result_iteration) % cycle_len;

                    if new_total < total_iterations {
                        memoization_cache.clear();
                        return roll_rocks_memoized(result_map, new_total, memoization_cache);
                    }
                }
                if direction == &Direction::North || direction == &Direction::South {
                    for col in 0..map_clone[0].len() {
                        if direction == &Direction::North {
                            for row in 0..map_clone.len() {
                                if map_clone[row][col] == 'O' {
                                    let mut rolldown_row = row;
                                    while rolldown_row > 0
                                        && map_clone[rolldown_row - 1][col] == '.'
                                    {
                                        map_clone[rolldown_row - 1][col] = 'O';
                                        map_clone[rolldown_row][col] = '.';
                                        rolldown_row -= 1;
                                    }
                                }
                            }
                        } else if direction == &Direction::South {
                            for row in (0..map_clone.len()).rev() {
                                if map_clone[row][col] == 'O' {
                                    let mut rolldown_row = row;
                                    while rolldown_row < map_clone.len() - 1
                                        && map_clone[rolldown_row + 1][col] == '.'
                                    {
                                        map_clone[rolldown_row + 1][col] = 'O';
                                        map_clone[rolldown_row][col] = '.';
                                        rolldown_row += 1;
                                    }
                                }
                            }
                        }
                    }
                } else if direction == &Direction::West || direction == &Direction::East {
                    for row in 0..map_clone.len() {
                        if direction == &Direction::West {
                            for col in 0..map_clone[0].len() {
                                if map_clone[row][col] == 'O' {
                                    let mut rolldown_col = col;
                                    while rolldown_col > 0
                                        && map_clone[row][rolldown_col - 1] == '.'
                                    {
                                        map_clone[row][rolldown_col - 1] = 'O';
                                        map_clone[row][rolldown_col] = '.';
                                        rolldown_col -= 1;
                                    }
                                }
                            }
                        } else if direction == &Direction::East {
                            for col in (0..map_clone[0].len()).rev() {
                                if map_clone[row][col] == 'O' {
                                    let mut rolldown_col = col;
                                    while rolldown_col < map_clone[0].len() - 1
                                        && map_clone[row][rolldown_col + 1] == '.'
                                    {
                                        map_clone[row][rolldown_col + 1] = 'O';
                                        map_clone[row][rolldown_col] = '.';
                                        rolldown_col += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            memoization_cache.insert(map.clone(), (map_clone.clone(), iteration));

            map = map_clone.clone();
        }

        if let Some(result) = memoization_cache.get(&(map.clone())) {
            result.0.clone()
        } else {
            map
        }
    }

    let mut cache: HashMap<Vec<Vec<char>>, (Vec<Vec<char>>, usize)> = HashMap::new();
    let rolled_map = roll_rocks_memoized(map, 1_000_000_000, &mut cache);

    //try to iterate, if rolled_map is not instantiated, then reutrn 0
    rolled_map
        .iter()
        .rev()
        .enumerate()
        .map(|(row, r)| r.iter().filter(|&&c| c == 'O').count() * (row + 1))
        .sum()
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
        assert_eq!(p1(input), 136);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 64);
    }
}
