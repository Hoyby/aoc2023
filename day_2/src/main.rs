// Day 2: Cube Conundrum

fn p1(input: &str) -> i32 {
    let game_constraint: [i32; 3] = [12, 13, 14]; // red, green, blue
    input
        .lines()
        .filter_map(|line| {
            let game: Vec<&str> = line.split(": ").collect();
            let game_id = game[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            let sets = game[1]
                .split("; ")
                .map(|set| {
                    let mut color_vec: Vec<i32> = vec![0; 3];
                    let colors: Vec<&str> = set.split(", ").collect();
                    colors.iter().for_each(|color| {
                        let color: Vec<&str> = color.split(" ").collect();
                        match color[1] {
                            "red" => color_vec[0] += color[0].parse::<i32>().unwrap(),
                            "green" => color_vec[1] += color[0].parse::<i32>().unwrap(),
                            "blue" => color_vec[2] += color[0].parse::<i32>().unwrap(),
                            _ => println!("Error"),
                        }
                    });
                    color_vec
                })
                .collect::<Vec<Vec<i32>>>();

            // check if any of the sets are greater than the game constraint, if so return None else return the game ID
            if sets.iter().any(|set| {
                set[0] > game_constraint[0]
                    || set[1] > game_constraint[1]
                    || set[2] > game_constraint[2]
            }) {
                None
            } else {
                Some(game_id)
            }
        })
        .sum::<i32>()
}

fn p2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let game: Vec<&str> = line.split(": ").collect();

            let mut max_elem_set: Vec<i32> = vec![0; 3];

            // parse each set to vec and return a vec containing the max value of each color
            game[1]
                .split("; ")
                .map(|set| {
                    let mut color_vec: Vec<i32> = vec![0; 3];
                    let colors: Vec<&str> = set.split(", ").collect();
                    colors.iter().for_each(|color| {
                        let color: Vec<&str> = color.split(" ").collect();
                        match color[1] {
                            "red" => color_vec[0] += color[0].parse::<i32>().unwrap(),
                            "green" => color_vec[1] += color[0].parse::<i32>().unwrap(),
                            "blue" => color_vec[2] += color[0].parse::<i32>().unwrap(),
                            _ => println!("Error"),
                        }
                    });
                    color_vec
                })
                .for_each(|set| {
                    max_elem_set
                        .iter_mut()
                        .zip(set.iter())
                        .for_each(|(max_elem, elem)| {
                            if elem > max_elem {
                                *max_elem = *elem;
                            }
                        });
                });

            // return the product of the max_elem_set
            Some(max_elem_set.iter().fold(1, |acc, elem| acc * elem))
        })
        .sum::<i32>()
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
        assert_eq!(p1(input), 8);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 2286);
    }
}
