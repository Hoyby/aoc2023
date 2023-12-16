// Day 12: Hot Springs

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spring {
    Broken,
    Working,
    Unknown,
}

fn p1_2(input: &str, repetitions: usize) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let springs_str = parts.next().unwrap();
            let groups_str = parts.next().unwrap();

            let mut springs = Vec::with_capacity((springs_str.len() + 1) * repetitions);
            let mut groups = Vec::with_capacity(groups_str.split(',').count() * repetitions);

            for _ in 0..repetitions {
                if !springs.is_empty() {
                    springs.push(Spring::Unknown);
                }
                springs.extend(springs_str.chars().map(|c| match c {
                    '#' => Spring::Broken,
                    '.' => Spring::Working,
                    '?' => Spring::Unknown,
                    _ => panic!(),
                }));
                groups.extend(
                    groups_str
                        .split(',')
                        .map(|num| num.parse::<usize>().unwrap()),
                );
            }

            let mut cache = vec![None; (groups.len() - 1) * springs.len()];
            let mut stack = Vec::new();
            let mut count = 0;
            let mut pos = 0;

            loop {
                let len: usize = groups[stack.len()];
                let end = pos + len;

                if end > springs.len() || (pos > 0 && springs[pos - 1] == Spring::Broken) {
                    if let Some((x, y)) = stack.pop() {
                        pos = x;
                        cache[stack.len() * springs.len() + pos] = Some(count);
                        count = y + count;
                        pos += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                if (end < springs.len() && springs[end] == Spring::Broken)
                    || springs[pos..end].iter().any(|&x| x == Spring::Working)
                {
                    pos += 1;
                    continue;
                }

                if stack.len() == groups.len() - 1 {
                    if springs[end..].iter().all(|&x| x != Spring::Broken) {
                        count += 1;
                    }
                    pos += 1;
                } else {
                    if let Some(old) = cache[stack.len() * springs.len() + pos] {
                        count += old;
                        pos += 1;
                    } else {
                        stack.push((pos, count));
                        count = 0;
                        pos = end + 1;
                    }
                }
            }

            count
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", p1_2(input, 1));
    println!("Part 2: {}", p1_2(input, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = include_str!("../example_1.txt");
        assert_eq!(p1_2(input, 1), 21);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p1_2(input, 5), 525152);
    }
}
