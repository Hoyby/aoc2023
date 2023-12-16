// Day 13: Point of Incidence

fn p1(input: &str) -> usize {
    let maps: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|map| {
            map.lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();

    let mut previous_offsets: Vec<usize> = Vec::new();

    for map in &maps {
        println!();
        let current_offsets: Vec<usize> = map
            .iter()
            .map(|line| {
                let line_rev: Vec<_> = line.iter().cloned().rev().collect();
                let mut potential_offsets: Vec<usize> = Vec::new();

                println!("1{:?}", line);
                println!("2{:?}", line_rev);

                for offset in 0..line_rev.len() {
                    let mut offset_ok = true;
                    for elem in offset..line.len() {
                        if line[elem] != line_rev[elem - offset] {
                            println!("break");
                            offset_ok = false;
                            break;
                        }
                        println!(
                            "o {} e {} {} {}",
                            offset,
                            elem,
                            line[elem],
                            line_rev[elem - offset]
                        );
                    }
                    if offset_ok {
                        potential_offsets.push(offset);
                        println!("potentials {:?}", potential_offsets);
                    }
                }

                potential_offsets
            })
            .fold(Vec::new(), |acc, x| {
                acc.into_iter()
                    .filter(|&offset| x.contains(&offset))
                    .collect()
            });

        previous_offsets = current_offsets;

        if previous_offsets.len() == 1 {
            break;
        }
    }

    previous_offsets[0]
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
        assert_eq!(p1(input), 374);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 8410);
    }
}
