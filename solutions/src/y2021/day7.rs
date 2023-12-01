pub struct Day7;

crate::impl_day!("7", true);

struct Positions(Vec<i32>);

fn process_input(input: &str) -> Positions {
    Positions(input.trim().split(',').flat_map(str::parse).collect())
}

fn solve(input: &str, folder: fn(i32, i32, i32) -> i32) -> i32 {
    let input = process_input(input);
    let min = *input.0.iter().min().unwrap();
    let max = *input.0.iter().max().unwrap();

    let mut min_fuel = i32::MAX;
    for i in min..=max {
        let current_min = input.0.iter().fold(0, |acc, n| folder(acc, *n, i));

        if current_min < min_fuel {
            min_fuel = current_min;
        }
    }

    min_fuel
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(solve(input, |acc, n, i| acc + (n - i).abs()))
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(solve(input, |acc, n, i| {
        acc + (1..=(n - i).abs()).sum::<i32>()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(37.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(168.to_string(), *solve_part2(INPUT).to_string());
    }
}
