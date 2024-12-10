use itertools::Itertools;

pub struct Day9;

crate::impl_day!("9", true);

fn process_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect_vec()
}

fn debug_print(disk_map: &[Option<i64>]) {
    for e in disk_map {
        if let Some(v) = e {
            print!("{v}");
        } else {
            print!(".");
        }
    }
    println!();
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut disk_map: Vec<Option<i64>> = vec![];
    let mut is_file = true;
    let mut i = 0;

    // Assemble the unsorted disk map
    for n in input {
        for _ in 0..n {
            if is_file {
                disk_map.push(Some(i));
            } else {
                disk_map.push(None);
            }
        }

        is_file = !is_file;
        if is_file {
            i += 1;
        }
    }

    // Rearrange the disk map
    let len = disk_map.len();

    loop {
        // debug_print(&disk_map);

        let (last_pos, &last) = disk_map
            .iter()
            .rev()
            .find_position(|e| e.is_some())
            .map(|(p, v)| (len - p - 1, v))
            .unwrap();
        let (first_pos, _) = disk_map.iter().find_position(|e| e.is_none()).unwrap();

        disk_map[last_pos] = None;
        disk_map[first_pos] = last;

        // Check if properly ordered
        let (last_some, _) = disk_map
            .iter()
            .rev()
            .find_position(|v| v.is_some())
            .map(|(n, v)| (len - n - 1, v))
            .unwrap();
        let (first_none, _) = disk_map.iter().find_position(|v| v.is_none()).unwrap();

        if last_some + 1 == first_none {
            break;
        }
    }

    let res = disk_map
        .iter()
        .filter(|e| e.is_some())
        .enumerate()
        .fold(0, |acc, (i, v)| acc + i as i64 * v.unwrap());

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

    const INPUT: &str = r"2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(1928.to_string(), *solve_part1(INPUT).to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0.to_string(), *solve_part2(INPUT).to_string());
    }
}
