// Day 6: Wait For It

fn p1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let time: Vec<i32> = lines[0]
        .split_whitespace()
        .skip(1) // Skip the "Time:" part
        .map(|s| s.parse().unwrap())
        .collect();

    let distance: Vec<i32> = lines[1]
        .split_whitespace()
        .skip(1) // Skip the "Distance:" part
        .map(|s| s.parse().unwrap())
        .collect();

    let num_ways: Vec<i32> = time
        .iter()
        .zip(distance.iter())
        .map(|(t, d)| {
            let mut result = 0;
            for travel_time in 0..*t {
                let crank = t - travel_time;
                let distance = crank * travel_time;
                if distance > *d {
                    result += 1;
                }
            }
            result
        })
        .collect();

    num_ways.iter().product()
}

fn p2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let time: i64 = lines[0]
        .split_whitespace()
        .skip(1) // Skip the "Time:" part
        .map(|s| s.parse::<i32>().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let distance: i64 = lines[1]
        .split_whitespace()
        .skip(1) // Skip the "Distance:" part
        .map(|s| s.parse::<i32>().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let mut result = 0;
    for travel_time in 0..time {
        let crank = time - travel_time;
        let my_distance = crank * travel_time;
        if my_distance > distance {
            result += 1;
        }
    }
    result
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
        assert_eq!(p1(input), 288);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 71503);
    }
}
