// Day 7: Camel Cards

use std::collections::{BTreeMap, HashMap};

fn p1(input: &str) -> u32 {
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    enum HandType {
        HighCard = 1,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    let card_values: HashMap<char, u32> = [
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('J', 9),
        ('T', 8),
        ('9', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]
    .iter()
    .cloned()
    .collect();

    let hands: BTreeMap<HandType, Vec<(&str, u32)>> = input
        .lines()
        .map(|line| {
            let mut hand = line.split_whitespace();
            let cards = hand.next().unwrap();
            let bid = hand.next().unwrap().parse::<u32>().unwrap();

            let mut hand_arr = [0; 13];
            for card in cards.chars() {
                match card_values.get(&card) {
                    Some(&value) => hand_arr[value as usize] += 1,
                    None => panic!("Invalid card"),
                }
            }

            let hand_type = match hand_arr.iter().max().unwrap() {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => {
                    if hand_arr.iter().filter(|&x| *x == 2).count() == 1 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                2 => {
                    if hand_arr.iter().filter(|&x| *x == 2).count() == 2 {
                        HandType::TwoPair
                    } else {
                        HandType::OnePair
                    }
                }
                1 => HandType::HighCard,
                _ => panic!("Invalid hand"),
            };

            (hand_type, (cards, bid))
        })
        .fold(BTreeMap::new(), |mut acc, x| {
            acc.entry(x.0).or_insert(Vec::new()).push(x.1);
            acc
        });

    let sorted_hands = hands
        .iter()
        .map(|(hand_type, cards_and_bids)| {
            let mut cards_and_bids = cards_and_bids.clone();
            cards_and_bids.sort_by(|a, b| {
                let a_hand: Vec<u32> = a.0.chars().map(|c| *card_values.get(&c).unwrap()).collect();
                let b_hand: Vec<u32> = b.0.chars().map(|c| *card_values.get(&c).unwrap()).collect();
                a_hand.cmp(&b_hand)
            });
            (hand_type, cards_and_bids)
        })
        .collect::<BTreeMap<_, _>>();

    sorted_hands
        .values()
        .flatten()
        .enumerate()
        .map(|(rank, hand)| hand.1 * (rank as u32 + 1))
        .sum::<u32>()
}

fn p2(input: &str) -> u32 {
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    enum HandType {
        HighCard = 1,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    let card_values: HashMap<char, u32> = [
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
        ('J', 0),
    ]
    .iter()
    .cloned()
    .collect();

    let hands: BTreeMap<HandType, Vec<(&str, u32)>> = input
        .lines()
        .map(|line| {
            let mut hand = line.split_whitespace();
            let cards = hand.next().unwrap();
            let bid = hand.next().unwrap().parse::<u32>().unwrap();

            let mut hand_arr = [0; 14];
            for card in cards.chars() {
                match card_values.get(&card) {
                    Some(&value) => hand_arr[value as usize] += 1,
                    None => panic!("Invalid card"),
                }
            }

            let amount_of_wild_cards = hand_arr[0];
            hand_arr[0] = 0;

            let (max_index, _max_elem) = hand_arr
                .iter()
                .enumerate()
                .max_by_key(|&(_, &x)| x)
                .unwrap();

            hand_arr[max_index] += amount_of_wild_cards;

            let hand_type = match hand_arr.iter().max().unwrap() {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => {
                    if hand_arr.iter().filter(|&x| *x == 2).count() == 1 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                2 => {
                    if hand_arr.iter().filter(|&x| *x == 2).count() == 2 {
                        HandType::TwoPair
                    } else {
                        HandType::OnePair
                    }
                }
                1 => HandType::HighCard,
                _ => panic!("Invalid hand"),
            };

            (hand_type, (cards, bid))
        })
        .fold(BTreeMap::new(), |mut acc, x| {
            acc.entry(x.0).or_insert(Vec::new()).push(x.1);
            acc
        });

    let sorted_hands = hands
        .iter()
        .map(|(hand_type, cards_and_bids)| {
            let mut cards_and_bids = cards_and_bids.clone();
            cards_and_bids.sort_by(|a, b| {
                let a_hand: Vec<u32> = a.0.chars().map(|c| *card_values.get(&c).unwrap()).collect();
                let b_hand: Vec<u32> = b.0.chars().map(|c| *card_values.get(&c).unwrap()).collect();
                a_hand.cmp(&b_hand)
            });
            (hand_type, cards_and_bids)
        })
        .collect::<BTreeMap<_, _>>();

    sorted_hands
        .values()
        .flatten()
        .enumerate()
        .map(|(rank, hand)| hand.1 * (rank as u32 + 1))
        .sum::<u32>()
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
        assert_eq!(p1(input), 6440);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 5905);
    }
}
