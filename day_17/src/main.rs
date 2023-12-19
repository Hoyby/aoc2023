// Day 17: Clumsy Crucible

use pathfinding::directed::astar::astar;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Node {
    pos: (i32, i32),
    dx: i32,
    dy: i32,
    num_straight: i32,
}

impl Node {
    fn successors(
        &self,
        map: &[&[u8]],
        w: i32,
        h: i32,
        is_ultra: bool,
    ) -> Vec<(Node, i32)> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(|&(dx, dy)| {
                if self.dx == -dx && self.dy == -dy {
                    return None;
                }
                let is_straight = self.dx == dx && self.dy == dy;
                let num_straight = if is_straight {
                    self.num_straight + 1
                } else {
                    1
                };
                if is_ultra {
                    if num_straight > 10 {
                        return None;
                    }
                    if (self.dx != 0 || self.dy != 0) && !is_straight && self.num_straight < 4 {
                        return None;
                    }
                } else if num_straight > 3 {
                    return None;
                }
                let (x, y) = (self.pos.0 + dx, self.pos.1 + dy);
                if x < 0 || x >= w || y < 0 || y >= h {
                    return None;
                }
                let heat_loss = (map[y as usize][x as usize] - b'0') as i32;
                Some((
                    Node {
                        pos: (x, y),
                        dx,
                        dy,
                        num_straight,
                    },
                    heat_loss,
                ))
            })
            .collect()
    }
    fn heuristic(&self, width: i32, height: i32, is_ultra: bool) -> i32 {
        (self.pos.0 - width + 1).abs()
            + (self.pos.1 - height + 1)
                .abs()
                .max(if is_ultra { 4 - self.num_straight } else { 0 })
    }
    fn success(&self, width: i32, height: i32, is_ultra: bool) -> bool {
        self.pos.0 == width - 1 && self.pos.1 == height - 1 && (!is_ultra || self.num_straight >= 4)
    }
}

fn p1(input: &str) -> usize {
    let heat_loss_map: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let width = heat_loss_map[0].len() as i32;
    let height = heat_loss_map[0].len() as i32;
    let (_, total_heat_loss) = astar(
        &Node::default(),
        |n| n.successors(&heat_loss_map, width, height, false),
        |n| n.heuristic(width, height, false),
        |n| n.success(width, height, false),
    )
    .unwrap();
    total_heat_loss as usize
}

fn p2(input: &str) -> usize {
    let heat_loss_map: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let width = heat_loss_map[0].len() as i32;
    let height = heat_loss_map.len() as i32;
    let mut min_heat_loss = 0;

    for is_ultra in [false, true] {
        let (_, total_heat_loss) = astar(
            &Node::default(),
            |n| n.successors(&heat_loss_map, width, height, is_ultra),
            |n| n.heuristic(width, height, is_ultra),
            |n| n.success(width, height, is_ultra),
        )
        .unwrap();
        min_heat_loss = total_heat_loss;
    }
    min_heat_loss as usize
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
        assert_eq!(p2(input), 94);
    }
}
