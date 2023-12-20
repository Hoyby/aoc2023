// Day 8: Haunted Wasteland

use std::collections::HashMap;

fn p1(input: &str) -> u32 {
    let mut parts = input.split("\n\n");
    let instructions: &str = parts.next().map(|ins| ins).unwrap();
    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();

            if parts.len() == 2 {
                let key = parts[0];
                let values = parts[1]
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(", ")
                    .collect::<Vec<&str>>();
                if values.len() == 2 {
                    Some((key, (values[0], values[1])))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut curr = "AAA";

    let mut count = 0;
    while curr != "ZZZ" {
        let instruction = &instructions
            .chars()
            .nth(count % instructions.len())
            .unwrap();
        if instruction == &'L' {
            curr = map.get(curr).unwrap().0;
        } else if instruction == &'R' {
            curr = map.get(curr).unwrap().1;
        }
        count += 1;
    }
    count as u32
}

fn p2(input: &str) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        match b {
            0 => a,
            _ => gcd(b, a % b),
        }
    }

    fn lcm(a: Vec<usize>) -> usize {
        let mut ans = 1;
        for i in a {
            ans = (ans * i) / gcd(ans, i);
        }
        ans
    }

    let mut parts = input.split("\n\n");
    let instructions: &str = parts.next().map(|ins| ins).unwrap();
    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();

            if parts.len() == 2 {
                let key = parts[0];
                let values = parts[1]
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(", ")
                    .collect::<Vec<&str>>();
                if values.len() == 2 {
                    Some((key, (values[0], values[1])))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut curr_vec: Vec<&str> = map
        .keys()
        .filter(|k: &&&str| k.ends_with('A'))
        .map(|k| *k)
        .collect();
    let mut tracker: HashMap<usize, usize> = HashMap::new();
    let mut count = 0;

    loop {
        let mut next_pos: Vec<&str> = Vec::new();

        for (i, curr) in curr_vec.iter().enumerate() {
            let new_curr = if instructions
                .chars()
                .nth(count % instructions.len())
                .unwrap()
                == 'L'
            {
                map.get(curr).unwrap().0
            } else {
                map.get(curr).unwrap().1
            };
            if new_curr.ends_with('Z') {
                tracker.insert(i, count + 1);
                if tracker.len() == curr_vec.len() {
                    return lcm(tracker.values().map(|v| *v).collect()) as usize;
                }
            }
            next_pos.push(new_curr);
        }

        curr_vec = next_pos;
        count += 1;
    }
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
        assert_eq!(p1(input), 6);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 7);
    }
}
