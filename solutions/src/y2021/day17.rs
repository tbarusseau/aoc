use regex::Regex;

pub struct Day17;

crate::impl_day!("17", true);

fn process_input(input: &str) -> (i32, i32, i32, i32) {
    let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();

    let cap = re.captures(input.trim()).unwrap();

    (
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].parse().unwrap(),
        cap[4].parse().unwrap(),
    )
}

fn step(pos: (i32, i32), velocity: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let new_pos = (pos.0 + velocity.0, pos.1 + velocity.1);

    let mut new_velocity = (velocity.0, velocity.1 - 1);
    if velocity.0 > 0 {
        new_velocity.0 = velocity.0 - 1;
    }

    // println!("[STEP] {pos:?}, {velocity:?}   ->   {new_pos:?}, {new_velocity:?}");

    (new_pos, new_velocity)
}

fn step_to_target_area(
    target_area: (i32, i32, i32, i32),
    velocity: (i32, i32),
    only_check_horizontal: bool,
) -> Option<(i32, i32)> {
    let mut pos = (0, 0);
    let mut velocity = velocity;

    loop {
        (pos, velocity) = step(pos, velocity);

        // Horizontal checks
        if pos.0 < target_area.0 && velocity.0 == 0 {
            // Will never reach, velocity_x == 0
            // println!("Breaking: will never reach, velocity_x == 0 && still not reached.");
            break;
        }

        if pos.0 > target_area.1 {
            // Went past the target area
            // println!("Breaking: went past target x.");
            break;
        }

        if !only_check_horizontal && pos.1 < target_area.3 {
            // Went below the target area, will never reach
            // println!("Breaking: went below the target area.");
            break;
        }

        #[allow(clippy::suspicious_operation_groupings)]
        if target_area.0 <= pos.0
            && pos.0 <= target_area.1
            && ((target_area.2 <= pos.1 && pos.1 <= target_area.3) || only_check_horizontal)
        {
            // println!("Found a valid initial velocity, returning");
            return Some(pos);
        }
    }

    None
}

#[allow(unused)]
fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let target_area = process_input(input);

    let mut fx = 0;

    let mut reached_area = false;
    let mut valid_starting_x = vec![];

    for x in 0.. {
        if step_to_target_area(target_area, (x, 0), true).is_some() {
            reached_area = true;
            valid_starting_x.push(x);
        } else if reached_area {
            break;
        }
    }

    let mut starting_velocities = vec![];

    for x in valid_starting_x {
        for y in 0.. {
            if step_to_target_area(target_area, (x, y), false).is_some() {
                starting_velocities.push((x, y));
            }
        }
    }

    println!("Valid velocities: {:?}", starting_velocities);

    let res = "Part 1 not done";
    Box::new(res)
}

#[allow(unused)]
fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);

    let res = "Part 2 not done";
    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"target area: x=20..30, y=-10..-5"#;

    #[test]
    fn test_part1() {
        assert_eq!(0.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
