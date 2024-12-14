use itertools::Itertools;
use regex::Regex;

pub struct Day14;

crate::impl_day!("14", true);

#[derive(Debug)]
struct Robot((isize, isize), (isize, isize));

fn process_input(input: &str) -> Vec<Robot> {
    let r = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    input
        .trim()
        .lines()
        .map(|l| {
            let captures = r.captures(l).unwrap();
            Robot(
                (
                    captures[1].parse::<isize>().unwrap(),
                    captures[2].parse::<isize>().unwrap(),
                ),
                (
                    captures[3].parse::<isize>().unwrap(),
                    captures[4].parse::<isize>().unwrap(),
                ),
            )
        })
        .collect_vec()
}

fn step(robots: &mut [Robot], width: isize, height: isize) {
    for robot in robots.iter_mut() {
        let mut new_pos = (
            (robot.0 .0 + robot.1 .0) % width,
            (robot.0 .1 + robot.1 .1) % height,
        );

        if new_pos.0 < 0 {
            new_pos.0 += width;
        }

        if new_pos.1 < 0 {
            new_pos.1 += height;
        }

        *robot = Robot(new_pos, robot.1);
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let width: isize = 101;
    let height: isize = 103;

    solve_with_dims(input, width, height, false)
}

fn display(robots: &[Robot], width: isize, height: isize) {
    for y in 0..height {
        for x in 0..width {
            if robots.iter().any(|Robot(pos, _)| pos.0 == x && pos.1 == y) {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }
    println!("----------------------------");
}

fn solve_with_dims(
    input: &str,
    width: isize,
    height: isize,
    display_results: bool,
) -> Box<dyn std::fmt::Display> {
    let mut robots = process_input(input);

    for i in 0..100 {
        if display_results && i > 7000 {
            println!("----------------------------");
            println!("Step: {i}");
            display(&robots, width, height);
        }

        step(&mut robots, width, height);
    }

    let mid_row = width / 2;
    let mid_height = height / 2;

    let qs = &mut [0, 0, 0, 0];
    for Robot(pos, _) in &robots {
        if pos.0 == mid_row || pos.1 == mid_height {
            continue;
        }

        if pos.0 < mid_row {
            if pos.1 < mid_height {
                qs[0] += 1;
            } else {
                qs[1] += 1;
            }
        } else if pos.1 < mid_height {
            qs[2] += 1;
        } else {
            qs[3] += 1;
        }
    }

    Box::new(qs.iter().product::<i32>())
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let width: isize = 101;
    let height: isize = 103;

    solve_with_dims(input, width, height, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(
            12.to_string(),
            *solve_with_dims(INPUT, 11, 7, false).to_string()
        );
    }
}
