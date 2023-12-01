pub struct Day6;

crate::impl_day!("6", true);

struct Fishes([u64; 9]);

fn process_input(input: &str) -> Fishes {
    let mut f = [0; 9];

    input.trim().split(',').for_each(|e| {
        let n = e.parse::<usize>().unwrap();
        f[n] += 1;
    });

    Fishes(f)
}

fn tick<'a>(fishes: &'a mut [u64; 9], swap: &'a mut [u64; 9]) {
    *swap = [0; 9];

    for (index, &count) in fishes.iter().enumerate() {
        if index == 0 {
            swap[6] += count;
            swap[8] += count;
        } else {
            swap[index - 1] += count;
        }
    }

    *fishes = *swap;
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut fishes = process_input(input);
    let mut new_fishes = [0; 9];

    for _ in 0..80 {
        tick(&mut fishes.0, &mut new_fishes);
    }

    let res: u64 = fishes.0.iter().sum();
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut fishes = process_input(input);
    let mut new_fishes = [0; 9];

    for _ in 0..256 {
        tick(&mut fishes.0, &mut new_fishes);
    }

    let res: u64 = fishes.0.iter().sum();
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(5934.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539_u64.to_string(), *solve_part2(INPUT).to_string());
    }
}
