use crate::solver::Solver;

pub struct Day12;

crate::impl_day!("12", true);

use num::integer::lcm;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Moon {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl Moon {
    pub fn from(other: &Moon) -> Moon {
        Moon {
            position: other.position,
            velocity: other.velocity,
        }
    }

    pub fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
        }
    }
}

#[derive(Debug)]
struct System {
    moons: Vec<Moon>,

    seen_x: HashSet<(i64, i64, i64, i64, i64, i64)>,
    seen_y: HashSet<(i64, i64, i64, i64, i64, i64)>,
    seen_z: HashSet<(i64, i64, i64, i64, i64, i64)>,

    rep_x: Option<u64>,
    rep_y: Option<u64>,
    rep_z: Option<u64>,

    cycle: u64,
}

impl System {
    pub fn from(input: Vec<Moon>) -> System {
        System {
            moons: input,
            seen_x: HashSet::new(),
            seen_y: HashSet::new(),
            seen_z: HashSet::new(),
            rep_x: None,
            rep_y: None,
            rep_z: None,
            cycle: 0,
        }
    }

    pub fn timestep(&mut self) -> Option<(u64, u64, u64)> {
        let old_moons = &self.moons;
        let mut new_moons: Vec<Moon> = old_moons.iter().map(Moon::from).collect();

        for moon in old_moons {
            new_moons = new_moons
                .iter_mut()
                .map(|m| {
                    let mut new = Moon::from(m);

                    #[allow(clippy::comparison_chain)]
                    if m.position.0 < moon.position.0 {
                        new.velocity.0 += 1;
                    } else if m.position.0 > moon.position.0 {
                        new.velocity.0 -= 1;
                    }

                    #[allow(clippy::comparison_chain)]
                    if m.position.1 < moon.position.1 {
                        new.velocity.1 += 1;
                    } else if m.position.1 > moon.position.1 {
                        new.velocity.1 -= 1;
                    }

                    #[allow(clippy::comparison_chain)]
                    if m.position.2 < moon.position.2 {
                        new.velocity.2 += 1;
                    } else if m.position.2 > moon.position.2 {
                        new.velocity.2 -= 1;
                    }

                    new
                })
                .collect();
        }

        new_moons = new_moons
            .iter_mut()
            .map(|m| {
                m.position.0 += m.velocity.0;
                m.position.1 += m.velocity.1;
                m.position.2 += m.velocity.2;

                *m
            })
            .collect();

        self.moons = new_moons;

        // This is for the second part of the exercise.
        // Note: This is hardcoded for three positions, and three moons. This doesn't scale with any problem.
        // The explanation is as follow:
        // - Each "dimension" of both the position and the velocity are independant, i.e. the three pos.x and the vel.x
        //   will never affect the other pos.y/z, vel.y/z.
        // - We only need to find a repetition for each "dimension" (i.e. x, y, and z).
        // - We can assume (this could have been false, rendering this solution useless) that the when the complete "history"
        //   repetition happens, it will be with the x/y/z repetitions we've already seen and stored.
        // - If that's the case, then the "history" will repeat itself at the only step where all of these are possible:
        //   LowestCommonMultiple(x, y, z) == LowestCommonMultiple(x, LowestCommonMultiple(y, z))
        if self.rep_x == None {
            match self.seen_x.contains(&(
                self.moons[0].position.0,
                self.moons[0].velocity.0,
                self.moons[1].position.0,
                self.moons[1].velocity.0,
                self.moons[2].position.0,
                self.moons[2].velocity.0,
            )) {
                true => {
                    self.rep_x = Some(self.cycle);
                }
                _ => {
                    self.seen_x.insert((
                        self.moons[0].position.0,
                        self.moons[0].velocity.0,
                        self.moons[1].position.0,
                        self.moons[1].velocity.0,
                        self.moons[2].position.0,
                        self.moons[2].velocity.0,
                    ));
                }
            }
        }

        if self.rep_y == None {
            match self.seen_y.contains(&(
                self.moons[0].position.1,
                self.moons[0].velocity.1,
                self.moons[1].position.1,
                self.moons[1].velocity.1,
                self.moons[2].position.1,
                self.moons[2].velocity.1,
            )) {
                true => {
                    self.rep_y = Some(self.cycle);
                }
                _ => {
                    self.seen_y.insert((
                        self.moons[0].position.1,
                        self.moons[0].velocity.1,
                        self.moons[1].position.1,
                        self.moons[1].velocity.1,
                        self.moons[2].position.1,
                        self.moons[2].velocity.1,
                    ));
                }
            }
        }

        if self.rep_z == None {
            match self.seen_z.contains(&(
                self.moons[0].position.2,
                self.moons[0].velocity.2,
                self.moons[1].position.2,
                self.moons[1].velocity.2,
                self.moons[2].position.2,
                self.moons[2].velocity.2,
            )) {
                true => {
                    self.rep_z = Some(self.cycle);
                }
                _ => {
                    self.seen_z.insert((
                        self.moons[0].position.2,
                        self.moons[0].velocity.2,
                        self.moons[1].position.2,
                        self.moons[1].velocity.2,
                        self.moons[2].position.2,
                        self.moons[2].velocity.2,
                    ));
                }
            }
        }

        if let (Some(a), Some(b), Some(c)) = (self.rep_x, self.rep_y, self.rep_z) {
            return Some((a, b, c));
        }

        self.cycle += 1;

        None
    }

    pub fn get_energy(&self) -> i64 {
        let mut r = 0;

        for moon in &self.moons {
            r += (moon.position.0.abs() + moon.position.1.abs() + moon.position.2.abs())
                * (moon.velocity.0.abs() + moon.velocity.1.abs() + moon.velocity.2.abs());
        }

        r
    }
}

fn solve_part1(_input: &str) -> Box<dyn std::fmt::Display> {
    let mut system = System::from(vec![
        Moon::new(-3, 10, -1),
        Moon::new(-12, -10, -5),
        Moon::new(-9, 0, 10),
        Moon::new(7, -5, -3),
    ]);

    for _ in 0..1000 {
        system.timestep();
    }

    let res = system.get_energy();

    Box::new(res)
}

fn solve_part2(_input: &str) -> Box<dyn std::fmt::Display> {
    let mut system = System::from(vec![
        Moon::new(-3, 10, -1),
        Moon::new(-12, -10, -5),
        Moon::new(-9, 0, 10),
        Moon::new(7, -5, -3),
    ]);

    let res = loop {
        match system.timestep() {
            Some((a, b, c)) => break lcm(lcm(a, b), c),
            _ => continue,
        }
    };

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut system = System::from(vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ]);

        for _ in 1..=10 {
            system.timestep();
        }
        assert_eq!(179, system.get_energy());

        let mut system = System::from(vec![
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ]);

        for _ in 1..=100 {
            system.timestep();
        }
        assert_eq!(1940, system.get_energy());
    }

    #[test]
    fn test_part2() {
        let mut system = System::from(vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ]);

        let r;
        loop {
            match system.timestep() {
                None => {
                    continue;
                }
                Some((a, b, c)) => {
                    r = lcm(lcm(a, b), c);
                    break;
                }
            }
        }

        assert_eq!(2772, r);

        let mut system = System::from(vec![
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ]);

        let r: u64;
        loop {
            match system.timestep() {
                None => {
                    continue;
                }
                Some((a, b, c)) => {
                    r = lcm(lcm(a, b), c);
                    break;
                }
            }
        }

        assert_eq!(4_686_774_924, r);
    }
}
