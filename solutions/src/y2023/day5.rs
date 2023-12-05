use itertools::Itertools;

pub struct Day5;

crate::impl_day!("5", true);

#[derive(Debug)]
struct Rules {
    seeds: Vec<i64>,
    maps: Vec<Vec<(i64, i64, i64)>>,
}

impl Rules {
    fn compute_next_location(n: i64, map: &(i64, i64, i64)) -> Option<i64> {
        let (dest_range, source_range, range_length) = *map;

        if n >= source_range && n < source_range + range_length {
            Some(n - source_range + dest_range)
        } else {
            None
        }
    }

    fn compute_final_location(&self, n: i64) -> i64 {
        self.maps.iter().fold(n, |acc, map| {
            for l in map {
                if let Some(v) = Self::compute_next_location(acc, l) {
                    return v;
                }
            }

            acc
        })
    }
}

fn process_input(input: &str) -> Rules {
    let mut seeds = vec![];
    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec![];

    let mut temp_map: Vec<(i64, i64, i64)> = vec![];

    for l in input.trim_end().lines().filter(|s| !s.is_empty()) {
        if l.starts_with("seeds: ") {
            seeds = l.split(' ').skip(1).flat_map(str::parse).collect_vec();

            continue;
        }

        if l.contains(':') {
            if !temp_map.is_empty() {
                maps.push(temp_map);
                temp_map = vec![];
            }

            continue;
        }

        temp_map.push(
            l.split(' ')
                .flat_map(str::parse::<i64>)
                .collect_tuple()
                .unwrap(),
        );
    }

    if !temp_map.is_empty() {
        maps.push(temp_map);
    }

    Rules { seeds, maps }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = input
        .seeds
        .iter()
        .map(|n| input.compute_final_location(*n))
        .inspect(|n| println!("{n}"))
        .min()
        .unwrap();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut min = i64::MAX;

    for mut chunk in &input.seeds.iter().chunks(2) {
        let &start = chunk.next().unwrap();
        let &length = chunk.next().unwrap();
        let end = start + length;

        for seed in start..end {
            min = min.min(input.compute_final_location(seed));
        }
    }

    Box::new(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part1() {
        assert_eq!(35.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(46.to_string(), *solve_part2(INPUT).to_string());
    }

    #[test]
    fn map_tests() {
        assert_eq!(Some(50), Rules::compute_next_location(98, &(50, 98, 2)));
        assert_eq!(Some(51), Rules::compute_next_location(99, &(50, 98, 2)));
        assert_eq!(None, Rules::compute_next_location(100, &(50, 98, 2)));

        assert_eq!(None, Rules::compute_next_location(45, &(52, 50, 48)));
        assert_eq!(Some(55), Rules::compute_next_location(53, &(52, 50, 48)));
    }
}
