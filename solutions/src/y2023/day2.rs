use std::convert::TryFrom;

use anyhow::anyhow;
use itertools::Itertools;

pub struct Day2;

crate::impl_day!("2", true);

struct Game {
    game_index: i32,
    subsets: Vec<(i32, i32, i32)>,
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut subsets = vec![];

        let colon_index = value.find(':').ok_or(anyhow!("colon not found"))?;
        let game_index_slice = &value[5..colon_index];
        let game_index: i32 = game_index_slice
            .parse()
            .map_err(|_| anyhow!("invalid game index"))?;

        let rest_slice = &value[colon_index + 2..];

        for subset in rest_slice.split("; ") {
            let mut red_count = 0;
            let mut blue_count = 0;
            let mut green_count = 0;

            for cube in subset.split(", ") {
                let space_index = cube.find(' ').ok_or(anyhow!("no space"))?;
                let count: i32 = cube[..space_index]
                    .parse()
                    .map_err(|_| anyhow!("invalid cube count"))?;
                let color = &cube[space_index + 1..];

                match color {
                    "red" => red_count += count,
                    "blue" => blue_count += count,
                    "green" => green_count += count,
                    _ => unreachable!(),
                };
            }

            subsets.push((red_count, blue_count, green_count));
        }

        Ok(Self {
            game_index,
            subsets,
        })
    }
}

fn process_input(input: &str) -> Vec<Game> {
    input
        .trim_end()
        .lines()
        .flat_map(Game::try_from)
        .collect_vec()
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let red_limit = 12;
    let blue_limit = 14;
    let green_limit = 13;

    let res = input.iter().fold(0, |acc, v| {
        for subset in &v.subsets {
            let (red, blue, green) = subset;

            let is_impossible = *red > red_limit || *blue > blue_limit || *green > green_limit;

            if is_impossible {
                return acc;
            }
        }

        acc + v.game_index
    });

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input.iter().fold(0, |acc, v| {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for subset in &v.subsets {
            let (red, blue, green) = subset;

            max_red = max_red.max(*red);
            max_blue = max_blue.max(*blue);
            max_green = max_green.max(*green);
        }

        acc + max_red * max_blue * max_green
    });

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part1() {
        assert_eq!(8.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286.to_string(), *solve_part2(INPUT).to_string());
    }
}
