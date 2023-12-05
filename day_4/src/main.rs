use std::collections::HashSet;

// input
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
fn p1(input: &str) -> i32 {
    input.lines()
        .filter_map(| card | {
            let card: Vec<&str> = card.split(": ").collect();
            let card_numbers: Vec<&str> = card[1].split(" | ").collect();
            let winning_numbers = card_numbers[0].split_whitespace().collect::<Vec<&str>>();
            let my_numbers = card_numbers[1].split_whitespace().collect::<Vec<&str>>();
            let intersection = winning_numbers.iter().filter(|&n| my_numbers.contains(n)).collect::<Vec<&&str>>();
            
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

// Task2: 
// you win copies of the scratchcards below the winning card equal to the number of matches. 
// So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.
// Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. 
// So, if you win a copy of card 10 and it has 5 matching numbers, 
// it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)
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
        let intersection = winning_numbers.iter().filter(|&n| my_numbers.contains(n)).collect::<Vec<&&str>>();
        
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