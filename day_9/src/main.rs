// Day 9: Mirage Maintenance

fn p1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let parsed_numbers = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>();

            fn predict_next_num(list: Vec<i64>) -> i64 {
                if list.iter().all(|&num| num == 0) {
                    return list.last().cloned().unwrap();
                }
                let next_list: Vec<i64> = list
                    .windows(2)
                    .map(|pair| pair[1].checked_sub(pair[0]).unwrap_or(0))
                    .collect();
                predict_next_num(next_list) + list.last().cloned().unwrap()
            }

            let ans = predict_next_num(parsed_numbers.clone());
            ans
        })
        .sum()
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
        assert_eq!(p1(input), 114);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 1);
    }
}
