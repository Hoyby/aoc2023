// Day 11: Cosmic Expansion

use std::collections::HashSet;

fn expanded_distance(
    coord1: i32,
    coord2: i32,
    galaxy_coords: &HashSet<i32>,
    expansion_factor: u64,
) -> u64 {
    let min_coord = coord1.min(coord2);
    let max_coord = coord1.max(coord2);
    let distance = (max_coord - min_coord) as u64;

    if distance < 2 {
        return distance;
    }

    let n_galaxies_in_between = galaxy_coords
        .iter()
        .filter(|&&coord| coord > min_coord && coord < max_coord)
        .count() as u64;

    (distance - 1 - n_galaxies_in_between) * expansion_factor + n_galaxies_in_between + 1
}

fn p1_2(input: &str, expansion_factor: u64) -> usize {
    let galaxies: Vec<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, c)| c == b'#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let galaxy_cols: HashSet<i32> = galaxies.iter().map(|&(x, _)| x).collect();
    let galaxy_rows: HashSet<i32> = galaxies.iter().map(|&(_, y)| y).collect();
    println!("{:?}", galaxy_cols);
    println!("{:?}", galaxy_rows);

    let total_distance: u64 = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy_i)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(move |galaxy_j| (galaxy_i, galaxy_j))
        })
        .flat_map(|((x1, y1), (x2, y2))| {
            vec![
                expanded_distance(*x1, *x2, &galaxy_cols, expansion_factor),
                expanded_distance(*y1, *y2, &galaxy_rows, expansion_factor),
            ]
        })
        .sum();
    total_distance as usize
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", p1_2(input, 2));
    println!("Part 2: {}", p1_2(input, 100_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = include_str!("../example_1.txt");
        assert_eq!(p1_2(input, 2), 374);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p1_2(input, 100), 8410);
    }
}
