use std::collections::HashMap;

use itertools::Itertools;
use md5::Digest;

pub struct Day14;

crate::impl_day!("14", true);

fn process_input(input: &str) -> &str {
    input.trim()
}

fn is_valid_p1(
    input: &str,
    index: i32,
    threeplets: &mut HashMap<Digest, char>,
    fiveplets: &mut HashMap<Digest, Vec<char>>,
) -> Option<(String, String)> {
    let digest = md5::compute(format!("{}{}", input, index));

    if let Some((s, c)) = is_threeplet(&digest, threeplets) {
        has_valid_stream(input, index, c, fiveplets, false)
            .map(|v| (s, v))
            .or(None)
    } else {
        None
    }
}

fn is_threeplet(input: &Digest, threeplets: &mut HashMap<Digest, char>) -> Option<(String, char)> {
    let hex = hex::encode(<[u8; 16]>::from(*input));

    if let Some(c) = threeplets.get(input) {
        return Some((hex, *c));
    }

    for (a, b, c) in hex.chars().tuple_windows() {
        if a == b && a == c {
            threeplets.insert(input.to_owned(), a);

            return Some((hex, a));
        }
    }

    None
}

fn has_valid_stream(
    input: &str,
    index: i32,
    needle: char,
    fiveplets: &mut HashMap<Digest, Vec<char>>,
    p2: bool,
) -> Option<String> {
    for i in 0..1000 {
        let mut digest = md5::compute(format!("{}{}", input, index + i + 1));

        if p2 {
            digest = get_stretched_hash(&digest);
        };

        let hex = hex::encode(<[u8; 16]>::from(digest));

        if let Some(v) = fiveplets.get(&digest) {
            if v.contains(&needle) {
                return Some(hex);
            }
        }

        for (a, b, c, d, e) in hex.chars().tuple_windows() {
            if a == b && a == c && a == d && a == e {
                fiveplets
                    .entry(digest)
                    .and_modify(|v| v.push(a))
                    .or_insert_with(|| vec![a]);

                if a == needle {
                    return Some(hex);
                }
            }
        }
    }

    None
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut index = 0;
    let mut count_found = 0;

    let mut threeplets: HashMap<Digest, char> = HashMap::new();
    let mut fiveplets: HashMap<Digest, Vec<char>> = HashMap::new();

    let res = loop {
        #[allow(unused)]
        if let Some(v) = is_valid_p1(input, index, &mut threeplets, &mut fiveplets) {
            // println!(
            //     "Key {count_found: >2} found at index {index: >6} | {} - {}",
            //     v.0, v.1
            // );

            count_found += 1;

            if count_found >= 64 {
                break index;
            }
        }

        index += 1;
    };

    Box::new(res)
}

fn get_stretched_hash(input: &Digest) -> Digest {
    // "Age" the hash...
    let mut old_digest = input.to_owned();
    let mut new_digest = None;

    for _ in 0..2016 {
        new_digest = Some(md5::compute(hex::encode(<[u8; 16]>::from(old_digest))));
        old_digest = new_digest.unwrap();
    }

    // println!("{input:?} => {new_digest:?}");

    new_digest.unwrap()
}

fn is_valid_p2(
    input: &str,
    index: i32,
    threeplets: &mut HashMap<Digest, char>,
    fiveplets: &mut HashMap<Digest, Vec<char>>,
) -> Option<(String, String)> {
    let digest = md5::compute(format!("{}{}", input, index));
    let aged_digest = get_stretched_hash(&digest);

    if let Some((s, c)) = is_threeplet(&aged_digest, threeplets) {
        has_valid_stream(input, index, c, fiveplets, true)
            .map(|v| (s, v))
            .or(None)
    } else {
        None
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let mut index = 0;
    let mut count_found = 0;

    let mut threeplets = HashMap::new();
    let mut fiveplets = HashMap::new();

    let res = loop {
        #[allow(unused)]
        if let Some(v) = is_valid_p2(input, index, &mut threeplets, &mut fiveplets) {
            // println!(
            //     "Key {count_found: >2} found at index {index: >6} | {} - {}",
            //     v.0, v.1
            // );

            count_found += 1;

            if count_found >= 64 {
                break index;
            }
        }

        index += 1;
    };

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"abc"#;

    #[test]
    fn test_part1() {
        assert_eq!(22728.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(22551.to_string(), *solve_part2(INPUT).to_string());
    }
}
