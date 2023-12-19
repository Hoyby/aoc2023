// Day 17: Clumsy Crucible

// A* search with manhattan distance heuristic
// Constraint: can max move 3 tiles at a time in a straight line, no diagonals
// The map is weightes, small numbers are easier to move through

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_draw_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    pos: (usize, usize),
    g_score: usize,
    h_score: usize,
    f_score: usize,
    parent: Option<Box<Node>>,
    direction: Option<Direction>,
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

fn a_star_shortest_path(
    start: (usize, usize),
    end: (usize, usize),
    map: &Vec<Vec<usize>>,
) -> Vec<Node> {
    let mut open: Vec<Node> = vec![Node {
        pos: start,
        g_score: 0,
        h_score: manhattan_distance(start, end),
        f_score: 0,
        parent: None,
        direction: None,
    }];
    let mut closed: HashSet<(usize, usize)> = HashSet::new();
    let mut path: Vec<Node> = vec![];

    while !open.is_empty() {
        let q = open
            .iter()
            .min_by(|a, b| a.f_score.cmp(&b.f_score))
            .unwrap()
            .clone();

        open.retain(|n| n != &q);

        closed.insert(q.pos);

        for (x, y) in vec![
            (q.pos.0 + 1, q.pos.1),
            (q.pos.0.checked_sub(1).unwrap_or(0), q.pos.1),
            (q.pos.0, q.pos.1 + 1),
            (q.pos.0, q.pos.1.checked_sub(1).unwrap_or(0)),
        ] {
            if x >= map.len() || y >= map[0].len() {
                continue;
            }
            let g_score = q.g_score + map[x][y];
            let h_score = manhattan_distance((x, y), end);
            let f_score = g_score + h_score;
            let direction = match (x as isize - q.pos.0 as isize, y as isize - q.pos.1 as isize) {
                (0, 1) => Some(Direction::Right),
                (0, -1) => Some(Direction::Left),
                (1, 0) => Some(Direction::Down),
                (-1, 0) => Some(Direction::Up),
                _ => None,
            };

            let node = Node {
                pos: (x, y),
                g_score,
                h_score,
                f_score,
                parent: Some(Box::new(q.clone())),
                direction,
            };

            // if new x, y is same as parent, skip
            if let Some(parent) = &node.parent {
                if parent.pos == node.pos {
                    continue;
                }
            }

            if node.pos == end {
                let mut current = &node;
                while let Some(parent) = &current.parent {
                    path.push(current.clone());
                    current = parent.as_ref();
                }
                path.reverse(); // Reverse the path to get it in the correct order
                return path;
            }

            if closed.contains(&node.pos) {
                continue;
            }

            // if current, parent and grandparent are on the same line, skip.
            if let Some(parent) = &node.parent {
                if let Some(grandparent) = &parent.parent {
                    if let Some(great_grandparent) = &grandparent.parent {
                        if let Some(direction) = &node.direction {
                            if let Some(parent_direction) = &parent.direction {
                                if let Some(grandparent_direction) = &grandparent.direction {
                                    if let Some(great_grandparent_direction) =
                                        &great_grandparent.direction
                                    {
                                        if direction == parent_direction
                                            && parent_direction == grandparent_direction
                                            && grandparent_direction == great_grandparent_direction
                                        {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if open.contains(&node) {
                let i = open.iter().position(|n| n == &node).unwrap();
                if open[i].f_score > node.f_score {
                    open[i] = node.clone();
                }
            } else {
                open.push(node.clone());
            }
        }
        closed.insert(q.pos);
    }

    path
}

fn p1(input: &str) -> usize {

    // TODO: Should work, but needs optimization
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect();

    let start = (0, 0);
    let end = (map.len() - 1, map[0].len() - 1);

    let path = a_star_shortest_path(start, end, &map);

    let mut trace_map = map
        .iter()
        .map(|line| line.iter().map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    path.iter().for_each(|node| {
        trace_map[node.pos.0][node.pos.1] = node
            .direction
            .as_ref()
            .unwrap_or(&Direction::Right)
            .to_draw_char();
    });

    println!();
    for line in &trace_map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    path.iter().map(|node| map[node.pos.0][node.pos.1]).sum()
}

fn p2(input: &str) -> usize {
    0
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
        assert_eq!(p1(input), 102);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 1);
    }
}
