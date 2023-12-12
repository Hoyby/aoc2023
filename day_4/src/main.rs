// Day 4: Scratchcards

fn p1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|card| {
            let card: Vec<&str> = card.split(": ").collect();
            let card_numbers: Vec<&str> = card[1].split(" | ").collect();
            let winning_numbers = card_numbers[0].split_whitespace().collect::<Vec<&str>>();
            let my_numbers = card_numbers[1].split_whitespace().collect::<Vec<&str>>();
            let intersection = winning_numbers
                .iter()
                .filter(|&n| my_numbers.contains(n))
                .collect::<Vec<&&str>>();

            let mut result = 1;

            for _ in 0..intersection.len() {
                result *= 2;
            }

            if result > 0 {
                Some(result / 2)
            } else {
                None
            }
        })
        .sum::<usize>() as i32
}

fn p2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut stack: Vec<usize> = (0..lines.len()).collect();
    stack.reverse();
    let mut total = 0;

    while let Some(i) = stack.pop() {
        total += 1;

        if i >= lines.len() {
            break;
        }

        let line = &lines[i];
        let card: Vec<&str> = line.split(": ").collect();
        let card_numbers: Vec<&str> = card[1].split(" | ").collect();

        let winning_numbers = card_numbers[0].split_whitespace().collect::<Vec<&str>>();
        let my_numbers = card_numbers[1].split_whitespace().collect::<Vec<&str>>();
        let intersection = winning_numbers
            .iter()
            .filter(|&n| my_numbers.contains(n))
            .collect::<Vec<&&str>>();

        println!("{:?}", winning_numbers);

        for j in (0..intersection.len()).rev() {
            if i + j + 1 < lines.len() {
                stack.push(i + j + 1);
            } else {
                break;
            }
        }
    }

    total
}

fn main() {
    let input = include_str!("../example_2.txt");
    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = include_str!("../example_1.txt");
        assert_eq!(p1(input), 13);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 30);
    }
}
