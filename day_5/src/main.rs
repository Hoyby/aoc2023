// Day 5: If You Give A Seed A Fertilizer

use std::ops::Range;

fn get_lookup_table<T>(map: T) -> Vec<(Range<i64>, i64)>
where
    T: AsRef<str>,
{
    map.as_ref()
        .lines()
        .skip(1)
        .map(|line| {
            let mut parts = line
                .split_whitespace()
                .filter_map(|part| part.parse::<i64>().ok());
            let dest_start = parts.next().unwrap();
            let src_start = parts.next().unwrap();
            let range = parts.next().unwrap();
            (src_start..src_start + range, dest_start - src_start)
        })
        .collect::<Vec<_>>()
}

fn p1(input: &str) -> i64 {
    let mut maps = input.split("\n\n");
    let mut seeds = maps
        .next()
        .map(|string| {
            string
                .trim_start_matches("seeds:")
                .split_whitespace()
                .filter_map(|item| item.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .unwrap();

    for map in maps {
        let table = get_lookup_table(map);
        seeds = seeds
            .into_iter()
            .map(|seed| {
                let mut source = seed;
                for (map_range, diff) in &table {
                    if source >= map_range.start && source < map_range.end {
                        source = map_range.start + diff + (source - map_range.start);
                        break;
                    }
                }
                source
            })
            .collect();
    }
    seeds.into_iter().min().unwrap()
}

fn p2(input: &str) -> i64 {
    let mut maps = input.split("\n\n");
    let mut seeds = maps
        .next()
        .map(|string| {
            string
                .trim_start_matches("seeds:")
                .split_whitespace()
                .filter_map(|item| item.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .unwrap()
        .chunks(2)
        .map(|chunk| {
            let arr = [chunk[0], chunk[1]];
            arr
        })
        .collect::<Vec<[i64; 2]>>();

    for map in maps {
        let table = get_lookup_table(map);

        let temp: Vec<[i64; 2]> = seeds
            .into_iter()
            .flat_map(|[start, mut range]| {
                range += start; // range is now end
                let mut temp = Vec::new();

                for (map_range, diff) in &table {
                    let isect_start = start.max(map_range.start);
                    let isect_end = range.min(map_range.end);

                    if isect_start < isect_end {
                        temp.push([isect_start + diff, isect_end - isect_start]);
                        if isect_start > start {
                            temp.push([map_range.start, isect_start - map_range.start]);
                        } else if range > isect_end {
                            temp.push([isect_end, range - isect_end]);
                        }
                        break;
                    }
                }
                if temp.is_empty() {
                    temp.push([start, range - start]);
                }
                temp
            })
            .collect();

        seeds = temp;
    }

    seeds.into_iter().min().unwrap()[0]
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
