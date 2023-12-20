// Day 15: Lens Library

fn compute_elf_hash(step: &[u8]) -> u32 {
    step.iter().fold(0, |current_value, &c| {
        ((current_value + c as u32) * 17) % 256
    })
}

fn apply_step(boxes: &mut [Vec<(String, u32)>], step: &str) {
    let step_bytes = step.as_bytes();
    let last_index = step_bytes.len() - 1;
    if step_bytes[last_index] == b'-' {
        let label_bytes = &step_bytes[..last_index];
        let hash = compute_elf_hash(label_bytes);
        let label = std::str::from_utf8(label_bytes).unwrap();
        boxes[hash as usize].retain(|(existing_label, _)| existing_label != label);
    } else {
        let mut parts = step.split('=');
        let label = parts.next().unwrap();
        let hash = compute_elf_hash(label.as_bytes());
        let focal_length: u32 = parts.next().unwrap().parse().unwrap();
        let old_position = boxes[hash as usize]
            .iter()
            .position(|(existing_label, _)| existing_label == label);
        if let Some(old_position) = old_position {
            boxes[hash as usize][old_position] = (label.to_string(), focal_length);
        } else {
            boxes[hash as usize].push((label.to_string(), focal_length));
        }
    }
}

fn total_focusing_power(boxes: &[Vec<(String, u32)>]) -> u32 {
    boxes
        .iter()
        .enumerate()
        .map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(slot_number, &(_, focal_length))| {
                    (box_number as u32 + 1) * (slot_number as u32 + 1) * focal_length
                })
                .sum::<u32>()
        })
        .sum()
}

fn p1(input: &str) -> usize {
    let sum_of_hashes: u32 = input
        .split(',')
        .map(|step| compute_elf_hash(step.as_bytes()))
        .sum();
    sum_of_hashes as usize
}

fn p2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    input
        .split(',')
        .for_each(|step| apply_step(&mut boxes, step));
    let sum_of_focusing_powers: u32 = total_focusing_power(&boxes);
    sum_of_focusing_powers as usize
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
        assert_eq!(p1(input), 1320);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../example_2.txt");
        assert_eq!(p2(input), 64);
    }
}
