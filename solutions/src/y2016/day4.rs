use std::{cmp::Ordering, collections::HashMap, convert::TryFrom};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day4;

crate::impl_day!("4", true);

#[derive(Debug)]
struct Room {
    name: String,
    id: i32,
    checksum: String,
}

lazy_static! {
    static ref ROOM_RE: Regex = Regex::new(r"(.*)-(\d+)\[(.*)\]").expect("invalid regex");
}

impl TryFrom<&str> for Room {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let captures = ROOM_RE
            .captures(value)
            .ok_or_else(|| "doesn't match regex")?;

        let name = captures
            .get(1)
            .ok_or_else(|| "no valid name")?
            .as_str()
            .to_owned();
        let id = captures
            .get(2)
            .map(|id| i32::from_str_radix(id.as_str(), 10))
            .ok_or_else(|| "id is not a number")?
            .map_err(|_| "no valid id")?;
        let checksum = captures
            .get(3)
            .ok_or_else(|| "no valid checksum")?
            .as_str()
            .to_owned();

        Ok(Room { name, id, checksum })
    }
}

fn process_input(input: &str) -> Vec<Room> {
    input.trim().lines().flat_map(Room::try_from).collect()
}

impl Room {
    pub fn is_valid(&self) -> bool {
        let sorted: String = self.name.replace('-', "").chars().sorted().collect();
        let mut dict: HashMap<char, i32> = HashMap::new();

        for c in sorted.chars() {
            dict.entry(c).and_modify(|v| *v += 1).or_default();
        }

        let valid_checksum: String = dict
            .iter()
            .sorted_by(|a, b| {
                let c1 = a.0;
                let v1 = a.1;
                let c2 = b.0;
                let v2 = b.1;

                if v1 > v2 {
                    Ordering::Less
                } else if v2 > v1 {
                    Ordering::Greater
                } else {
                    c1.cmp(c2)
                }
            })
            .take(5)
            .map(|e| e.0.to_owned())
            .collect();

        valid_checksum == self.checksum
    }

    pub fn decypher_name(&self) -> String {
        let rot = self.id % 26;
        let mut decyphered_name = String::new();

        for c in self.name.chars() {
            if c == '-' {
                decyphered_name.push(' ');
                continue;
            }

            decyphered_name.push(((((c as u8 - 'a' as u8) + rot as u8) % 26) + ('a' as u8)) as char)
        }

        decyphered_name
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let rooms = process_input(input);

    let res = rooms
        .iter()
        .filter(|r| Room::is_valid(r))
        .fold(0, |acc, x| acc + x.id);
    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let rooms = process_input(input);

    let res = rooms
        .iter()
        .map(|r| (r.decypher_name(), r.id))
        .filter(|n| n.0 == "northpole object storage")
        .next()
        .expect("Not found")
        .1;

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(Room::try_from("aaaaa-bbb-z-y-x-123[abxyz]")
            .unwrap()
            .is_valid());
        assert!(Room::try_from("a-b-c-d-e-f-g-h-987[abcde]")
            .unwrap()
            .is_valid());
        assert!(Room::try_from("not-a-real-room-404[oarel]")
            .unwrap()
            .is_valid());
        assert!(!Room::try_from("totally-real-room-200[decoy]")
            .unwrap()
            .is_valid());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Room::try_from("qzmt-zixmtkozy-ivhz-343[abcde]")
                .unwrap()
                .decypher_name(),
            "very encrypted name"
        )
    }
}
