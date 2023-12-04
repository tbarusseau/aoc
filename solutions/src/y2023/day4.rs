use itertools::Itertools;

pub struct Day4;

crate::impl_day!("4", true);

#[derive(Debug)]
struct Game {
    winning: Vec<String>,
    rolled: Vec<String>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let colon_index = value.find(':').unwrap();

        let mut map = value[colon_index + 1..].split(" | ").map(|s| {
            s.trim()
                .chars()
                .filter(|c| c.is_numeric() || c.is_whitespace())
                .collect::<String>()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(std::borrow::ToOwned::to_owned)
                .collect_vec()
        });

        let winning = map.next().unwrap();
        let rolled = map.next().unwrap();

        Self { winning, rolled }
    }
}

impl Game {
    fn count_wins(&self) -> usize {
        self.rolled.iter().fold(0, |acc, v| {
            if self.winning.contains(v) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn process_input(input: &str) -> Vec<Game> {
    input.lines().map(Game::from).collect_vec()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input.iter().fold(0, |acc, g| {
        let wins = g.count_wins();

        if wins == 0 {
            return acc;
        }

        acc + 2_i32.pow((wins - 1) as u32)
    });

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let games = process_input(input);

    let mut card_counts = vec![1u32];
    let mut count = 0;

    for (n, game) in games.iter().enumerate() {
        let wins = game.count_wins();
        let end = wins + n + 1;

        if end > card_counts.len() {
            card_counts.resize(end, 1);
        }

        for i in n + 1..end {
            card_counts[i] += card_counts[n];
        }

        count += card_counts[n];
    }

    Box::new(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(13.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(30.to_string(), *solve_part2(INPUT).to_string());
    }
}
