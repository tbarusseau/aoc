use itertools::Itertools;

pub struct Day9;

crate::impl_day!("9", true);

fn process_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| i64::from(c.to_digit(10).unwrap()))
        .collect_vec()
}

fn compute_checksum(disk_map: &[Option<i64>]) -> i64 {
    disk_map
        .iter()
        .enumerate()
        .filter(|(_, e)| e.is_some())
        .fold(0, |acc, (i, v)| acc + i as i64 * v.unwrap())
}

fn assemble_disk_map(input: &[i64]) -> Vec<Option<i64>> {
    let mut disk_map: Vec<Option<i64>> = vec![];
    let mut is_file = true;
    let mut i = 0;

    for &n in input {
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

    disk_map
}

fn rearrange_disk_map(disk_map: &mut [Option<i64>]) {
    let len = disk_map.len();

    loop {
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
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut disk_map = assemble_disk_map(&input);
    rearrange_disk_map(&mut disk_map);

    Box::new(compute_checksum(&disk_map))
}

fn defragment_disk_map(disk_map: &mut [Option<i64>]) {
    let last_index = disk_map.last().unwrap().unwrap();

    for file_index in (0..=last_index).rev() {
        // Compute file length
        let (first_index, _) = disk_map
            .iter()
            .find_position(|&v| v.is_some() && v.unwrap() == file_index)
            .expect("couldn't find first index");
        let last_index = disk_map
            .iter()
            .rev()
            .find_position(|v| v.is_some() && v.unwrap() == file_index)
            .map(|(index, _)| disk_map.len() - index - 1)
            .expect("couldn't find last index");

        let file_len = last_index - first_index + 1;

        // Find first gap that fits the file
        let mut pos = None;

        'outer: for i in 0..disk_map.len() {
            for j in 0..file_len {
                if i + j >= disk_map.len() {
                    break;
                }

                let current = disk_map[i + j];

                if current.is_some() {
                    break;
                }

                if j == file_len - 1 && i < first_index {
                    pos = Some(i);
                    break 'outer;
                }
            }
        }

        if let Some(pos) = pos {
            // Delete the file first
            for i in first_index..=last_index {
                disk_map[i] = None;
            }

            // Move it at the found location
            for i in 0..file_len {
                disk_map[pos + i] = Some(file_index);
            }
        }
    }
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let input = process_input(input);
    let mut disk_map = assemble_disk_map(&input);
    defragment_disk_map(&mut disk_map);

    Box::new(compute_checksum(&disk_map))
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
        assert_eq!(2858.to_string(), *solve_part2(INPUT).to_string());
    }
}
