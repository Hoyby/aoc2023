// Day 5: If You Give A Seed A Fertilizer

use rayon::prelude::*;
use std::ops::Range;

#[derive(Clone)]
struct Map {
    source_range: Range<usize>,
    destination_range: Range<usize>,
}

fn transform_seed_from_start_to_end(all_maps: Vec<Vec<Map>>, seed: usize) -> usize {
    let mut result = seed;

    for map in all_maps {
        result = map
            .iter()
            .find(|subsection| subsection.source_range.contains(&result))
            .map(|subsection| {
                subsection.destination_range.start + (result - subsection.source_range.start)
            })
            .unwrap_or(result);
    }

    result
}

fn parse_subsections(section: &str) -> Vec<Map> {
    section
        .split_once(":\n")
        .unwrap()
        .1
        .lines()
        .map(str::split_whitespace)
        .map(|line| line.map(str::parse).map(Result::unwrap).collect::<Vec<_>>())
        .map(|nums| Map {
            source_range: nums[1]..nums[1] + nums[2],
            destination_range: nums[0]..nums[0] + nums[2],
        })
        .collect()
}

fn p1(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let maps = parts[1..]
        .iter()
        .map(|section| parse_subsections(section))
        .collect::<Vec<Vec<Map>>>();

    let seeds = parts[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    seeds
        .map(|seed| transform_seed_from_start_to_end(maps.clone(), seed))
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let maps = parts[1..]
        .iter()
        .map(|section| parse_subsections(section))
        .collect::<Vec<Vec<Map>>>();

    let seeds = parts[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .fold(Vec::new(), |mut acc, chunk| {
            for i in chunk[0]..chunk[0] + chunk[1] {
                acc.push(i);
            }

            acc
        });

    seeds
        .into_par_iter()
        .map(|seed| transform_seed_from_start_to_end(maps.clone(), seed))
        .min()
        .unwrap()
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
        assert_eq!(p2(input), 46);
    }
}
