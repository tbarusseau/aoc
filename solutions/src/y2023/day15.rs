use itertools::Itertools;

pub struct Day15;

crate::impl_day!("15", true);

fn process_input(input: &str) -> Vec<&str> {
    input.trim_end().split(',').collect_vec()
}

fn compute_hash_algorithm(input: &str) -> usize {
    input
        .as_ascii()
        .expect("not valid ASCII")
        .iter()
        .fold(0, |acc, v| {
            let ascii_code = v.to_u8();

            ((acc + ascii_code as usize) * 17) % 256
        })
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let res: usize = process_input(input)
        .iter()
        .map(|s| compute_hash_algorithm(s))
        .sum();

    Box::new(res)
}

fn compute_focusing_power(input: &[Vec<(String, usize)>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(box_index, v)| {
            v.iter()
                .enumerate()
                .map(|(slot_index, v)| (box_index + 1) * (slot_index + 1) * v.1)
                .sum::<usize>()
        })
        .sum()
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for s in process_input(input) {
        let Some(stop_index) = s.find(['=', '-']) else {
            unreachable!("invalid input: {}", s)
        };

        let label = &s[0..stop_index];
        let instruction = &s[stop_index..=stop_index];

        let hash = compute_hash_algorithm(label);
        let pos = boxes[hash].iter().position(|v| v.0 == label);

        match instruction {
            "=" => {
                let focal_length = (s[stop_index + 1..stop_index + 2])
                    .parse::<usize>()
                    .unwrap();

                let entry = (label.to_owned(), focal_length);

                if let Some(index) = pos {
                    boxes[hash][index] = entry;
                } else {
                    boxes[hash].push(entry);
                }
            }
            "-" => {
                if let Some(index) = pos {
                    boxes[hash].remove(index);
                }
            }
            _ => unreachable!(),
        }
    }

    let res = compute_focusing_power(&boxes);

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(52, compute_hash_algorithm("HASH"));
        assert_eq!(1320.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(145.to_string(), *solve_part2(INPUT).to_string());
    }
}
