
// Day 5: If You Give A Seed A Fertilizer

use std::{collections::{BTreeMap}, num::ParseIntError};

fn p1(input: &str) -> u32 {
    let mut maps: BTreeMap<&str, Vec<Vec<u64>>> = BTreeMap::new();
    let mut current_map: Option<&str> = None;
    let mut map_order: Vec<&str> = Vec::new();

    let seeds: Vec<u64> = input
        .lines()
        .next() // Get the first line
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap()) // Parse each seed to u32
        .collect();

    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if line.ends_with(":") {
                current_map = Some(line.trim_end_matches(" map:"));
                maps.insert(current_map.unwrap(), Vec::new());
                map_order.push(current_map.unwrap());
            } else {
                let numbers: Result<Vec<u64>, ParseIntError> = line
                    .split_whitespace()
                    .map(|s| s.parse())
                    .collect();
                let numbers = match numbers {
                    Ok(n) => n,
                    Err(_) => return, // Skip lines with parsing errors
                };

                let map_data_vec = vec![numbers[0], numbers[1], numbers[2]];


                // insert a tripple vector into the map
                maps.get_mut(current_map.unwrap()).unwrap().push(map_data_vec);
            }
        });

    seeds.iter()
        .map(|seed| {
            let mut source = *seed as u64;
            map_order.iter()
                .for_each(|map| {
                    println!("Map: {}", map);
                    let map_data = maps.get(map).unwrap();
                    for data in map_data {
                        if source >= data[1] as u64 && source < (data[1] as u64 + data[2] as u64) {
                            println!("Source changed from {} to {}", source, data[0] as u64 + (source - data[1] as u64));
                            source = data[0] as u64 + (source - data[1] as u64);
                            break;
                        }
                    }
                });
            println!("Final source: {}", source);
            source
        })
        .min()
        .unwrap() as u32
}



fn p2(_input: &str) -> i32 {
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
        assert_eq!(p1(input), 35);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 30);
    }
}