use itertools::Itertools;

pub struct Day7;

crate::impl_day!("7", true);

fn process_input(input: &str) -> Vec<(String, i32)> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let first = split.next().unwrap().to_owned();
            let second = split.next().unwrap().parse().unwrap();

            (first, second)
        })
        .collect_vec()
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_power_ranks(part1: bool) -> &'static [char] {
    const POWERS_P1: &[char] = &[
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    const POWERS_P2: &[char] = &[
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    if part1 {
        POWERS_P1
    } else {
        POWERS_P2
    }
}

impl Type {
    fn from_str(value: &str, part1: bool) -> Self {
        let power_ranks = get_power_ranks(part1);
        let mut counts = [0; 13];

        if part1 {
            let as_values = value
                .chars()
                .map(|c| power_ranks.iter().position(|&d| d == c).unwrap())
                .collect_vec();

            as_values.iter().for_each(|&v| counts[v] += 1);
        } else {
            let jokers_count = value.chars().filter(|&c| c == 'J').count();

            // Remove the jokers from the pool
            let as_values = value
                .chars()
                .filter(|&c| c != 'J')
                .map(|c| power_ranks.iter().position(|&d| d == c).unwrap())
                .collect_vec();

            as_values.iter().for_each(|&v| counts[v] += 1);

            // Add the jokers to the biggest count of the biggest card
            let max = counts.iter().max().expect("no maximum");
            let last_max_index = counts
                .iter()
                .enumerate()
                .filter(|(_, v)| *v == max)
                .last()
                .map(|(i, _)| i)
                .unwrap();

            counts[last_max_index] += jokers_count;
        }

        if counts.iter().any(|&v| v == 5) {
            Self::FiveOfAKind
        } else if counts.iter().any(|&v| v == 4) {
            Self::FourOfAKind
        } else if counts.iter().any(|&v| v == 3) && counts.iter().any(|&v| v == 2) {
            Self::FullHouse
        } else if counts.iter().any(|&v| v == 3) {
            Self::ThreeOfAKind
        } else if counts.iter().filter(|&v| *v == 2).count() == 2 {
            Self::TwoPair
        } else if counts.iter().any(|&v| v == 2) {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

fn compare_hands(a: &str, b: &str, part1: bool) -> std::cmp::Ordering {
    let a_type = Type::from_str(a, part1);
    let b_type = Type::from_str(b, part1);

    let power_ranks = get_power_ranks(part1);

    match a_type.cmp(&b_type) {
        // If equal, order by first non-equal power rank ordering.
        std::cmp::Ordering::Equal => a
            .chars()
            .zip(b.chars())
            .map(|(a, b)| {
                (
                    power_ranks.iter().position(|c| *c == a).unwrap(),
                    power_ranks.iter().position(|c| *c == b).unwrap(),
                )
            })
            .map(|(a, b)| a.cmp(&b))
            .find(|r| r.is_ne())
            .expect("no ordering found"),
        // If not equal, order by type ordering.
        v => v,
    }
}

fn solve(input: &[(String, i32)], part1: bool) -> i32 {
    input
        .iter()
        .sorted_by(|(a, _), (b, _)| compare_hands(a, b, part1))
        .enumerate()
        .fold(0, |acc, v| {
            let (i, (_, bid)) = v;

            acc + (i as i32 + 1) * *bid
        })
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let res = solve(&input, true);

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let res = solve(&input, false);

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part1() {
        assert_eq!(6440.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(5905.to_string(), *solve_part2(INPUT).to_string());
    }
}
