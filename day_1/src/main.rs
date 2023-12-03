// Day 1: Trebuchet?!

fn p1(input: &str) -> i32 {
    input.lines()
        .filter_map(|line| {
            // filter only digits
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>();

            let first = digits.first().unwrap_or(&'0').to_owned();
            let last = digits.last().unwrap_or(&'0').to_owned();
            let number = format!("{}{}", first, last)
                .parse::<i32>()
                .ok();
            number
        })
        .sum::<i32>()
}


fn p2(input: &str) -> i32 {

    fn find_all_indexes(line: &str, word: &str) -> Vec<usize> {
        let mut start = 0;
        let mut indexes = Vec::new();
    
        while let Some(index) = line[start..].find(word) {
            indexes.push(start + index);
            start += index + word.len();
        }
    
        indexes
    }

    let word_number= ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];

    input.lines()
        .filter_map(|line| {
            let v = word_number.iter()
                .flat_map(|&word| {
                    if line.contains(word) {
                        let indexes = find_all_indexes(line, word);
                        indexes.into_iter().map(move |index| (word_number.iter().position(|&r| r == word).unwrap() + 1, index)).collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    }
                })
                .chain(
                    line.chars()
                        .filter(|c| c.is_ascii_digit())
                        .flat_map(|c| {
                            let indexes = find_all_indexes(line, &c.to_string());
                            indexes.into_iter().map(move |index| (c.to_digit(10).unwrap() as usize, index)).collect::<Vec<_>>()
                        })
                )
                .collect::<Vec<(usize, usize)>>();
            
            let mut v = v.clone();
            v.sort_by(|a, b| a.1.cmp(&b.1));

            let first = v.first().unwrap_or(&(0, 0)).to_owned();
            let last = v.last().unwrap_or(&(0, 0)).to_owned();
            let number = format!("{}{}", first.0, last.0)
                .parse::<i32>()
                .ok();
            number
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
    fn p_1() {
        assert_eq!(p1(include_str!("../example_1.txt")), 142);
    }

    #[test]
    fn p_2() {
        assert_eq!(p2(include_str!("../example_2.txt")), 281);
    }
}