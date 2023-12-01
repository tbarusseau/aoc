use itertools::Itertools;

pub struct Day8;

crate::impl_day!("8", true);

fn process_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut split = l.split(" | ");

            let signals: Vec<&str> = split.next().unwrap().split(' ').collect();
            let digits: Vec<&str> = split.next().unwrap().split(' ').collect();

            (signals, digits)
        })
        .collect()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut res_count = 0;

    for (_, digits) in input {
        for digit in digits {
            let len = digit.len();
            if len == 2 || len == 4 || len == 3 || len == 7 {
                res_count += 1;
            }
        }
    }

    Box::new(res_count)
}

const DIGITS: [&str; 10] = [
    "abcdeg", "ab", "acdfg", "abcdf", "abef", "bcdef", "bcdefg", "abd", "abcdefg", "abcdef",
];

fn create_digit(perm: &[char], signal: &str) -> Option<usize> {
    let decoded = signal
        .chars()
        .map(|c| perm[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();

    DIGITS.iter().position(|&s| s == decoded.as_str())
}

fn try_permutation(
    permutation: &[char],
    (signals, digits): &(Vec<&str>, Vec<&str>),
) -> Option<usize> {
    let invalid = signals
        .iter()
        .map(|s| create_digit(permutation, s))
        .any(|r| r.is_none());

    if invalid {
        return None;
    }

    let answer = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, s)| create_digit(permutation, s).unwrap() * 10_usize.pow(i as u32))
        .sum();

    Some(answer)
}

fn bruteforce_line(line: &(Vec<&str>, Vec<&str>)) -> usize {
    "abcdefg"
        .chars()
        .permutations(7)
        .find_map(|permutation| try_permutation(&permutation, line))
        .unwrap()
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res: usize = input.iter().map(bruteforce_line).sum();

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(26.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(61229.to_string(), *solve_part2(INPUT).to_string());
    }
}
