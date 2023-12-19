// Day 13: Point of Incidence

fn count_reflection_errors(
    grid: &[Vec<bool>],
    mirror_index: usize,
    is_horizontal: bool,
    max_errors: u32,
) -> u32 {
    let mut num_errors = 0;
    let (size_along_mirror, size_perpendicular) = if is_horizontal {
        (grid[0].len(), grid.len())
    } else {
        (grid.len(), grid[0].len())
    };
    let num_reflected = (mirror_index + 1).min(size_perpendicular - mirror_index - 1);
    for i in 0..size_along_mirror {
        for j in 0..num_reflected {
            let (pos, pos_reflected) = if is_horizontal {
                ((i, mirror_index - j), (i, mirror_index + j + 1))
            } else {
                ((mirror_index - j, i), (mirror_index + j + 1, i))
            };
            if grid[pos.1][pos.0] != grid[pos_reflected.1][pos_reflected.0] {
                num_errors += 1;
                if num_errors > max_errors {
                    return num_errors;
                }
            }
        }
    }
    num_errors
}

fn find_reflection_id(grid: &[Vec<bool>], num_reflection_errors: u32) -> u32 {
    for (is_horizontal, multiplier, size_perpendicular) in
        [(false, 1, grid[0].len()), (true, 100, grid.len())]
    {
        for mirror_index in 0..size_perpendicular - 1 {
            let num_errors =
                count_reflection_errors(grid, mirror_index, is_horizontal, num_reflection_errors);
            if num_errors == num_reflection_errors {
                return (mirror_index as u32 + 1) * multiplier;
            }
        }
    }
    unreachable!();
}

fn p1(input: &str) -> usize {
    let grids: Vec<Vec<Vec<bool>>> = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.bytes().map(|c| c == b'#').collect())
                .collect()
        })
        .collect();

    let total_reflection_ids: u32 = grids
        .iter()
        .map(|grid| find_reflection_id(grid, 0))
        .sum();

    total_reflection_ids as usize
}

fn p2(input: &str) -> usize {
    let grids: Vec<Vec<Vec<bool>>> = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.bytes().map(|c| c == b'#').collect())
                .collect()
        })
        .collect();

    let total_reflection_ids: u32 = grids
        .iter()
        .map(|grid| find_reflection_id(grid, 1))
        .sum();

    total_reflection_ids as usize
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
        assert_eq!(p1(input), 405);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 400);
    }
}




