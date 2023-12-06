fn p1(input: &str) -> i32 {
    0
}


fn p2(input: &str) -> i32 {
    0
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