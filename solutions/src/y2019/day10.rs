pub struct Day10;

crate::impl_day!("10", true);

use num::integer::gcd;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Asteroid {
    x: i32,
    y: i32,
}

struct AsteroidsField {
    width: i32,
    height: i32,
    asteroids: Vec<Option<Asteroid>>,
    starting_position: Option<(isize, isize)>,
    pub visibility: Vec<i32>,
}

fn normalize(x: u32, y: u32) -> (i64, i64) {
    let g = gcd(x, y);
    match g {
        0 => (i64::from(x), i64::from(y)),
        _ => (i64::from(x / g), i64::from(y / g)),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Frac {
    numerator: isize,
    denominator: isize,
}

// Outputs all the possible fractions between 0 and 1, going as low as `1 / n`
fn farey_sequence(n: isize) -> Vec<Frac> {
    let mut out = Vec::new();
    let mut f1 = Frac {
        numerator: 0,
        denominator: 1,
    };
    let mut f2 = Frac {
        numerator: 1,
        denominator: n,
    };

    out.push(f1);
    out.push(f2);

    while f2.denominator > 1 {
        let k = (n + f1.denominator) / f2.denominator;
        let t = f1;
        f1 = f2;
        f2 = Frac {
            numerator: f2.numerator * k - t.numerator,
            denominator: f2.denominator * k - t.denominator,
        };
        out.push(f2);
    }

    out
}

impl AsteroidsField {
    pub fn from(input: &str) -> Self {
        let mut asteroids: Vec<Option<Asteroid>> = Vec::new();
        let input: Vec<&str> = input.lines().collect();
        let height = input.len();
        let width = input[0].len();

        (0..height).for_each(|y| {
            for x in 0..width {
                asteroids.push(match input[y].chars().nth(x) {
                    Some('#') => Some(Asteroid {
                        x: x as i32,
                        y: y as i32,
                    }),
                    _ => None,
                });
            }
        });

        Self {
            width: width as i32,
            height: height as i32,
            asteroids,
            visibility: Vec::new(),
            starting_position: None,
        }
    }

    // Use a hashmap to cheat part1, get distance from current position, normalize it, and place 1 at its hashmap entry.
    pub fn compute_visibility(&mut self) {
        for y1 in 0..self.width {
            for x1 in 0..self.height {
                let a = &self.asteroids[y1 as usize * self.width as usize + x1 as usize];

                match a {
                    None => self.visibility.push(0),
                    Some(_) => {
                        let mut map: HashMap<(i64, i64), u32> = HashMap::new();
                        for y2 in 0..self.height {
                            for x2 in 0..self.width {
                                let other = &self.asteroids
                                    [y2 as usize * self.width as usize + x2 as usize];

                                match other {
                                    None => continue,
                                    Some(_) => {
                                        let x = x2 - x1;
                                        let y = y2 - y1;
                                        let sx = x < 0;
                                        let sy = y < 0;

                                        let (mut x, mut y) =
                                            normalize(x.unsigned_abs(), y.unsigned_abs());
                                        if sx {
                                            x *= -1;
                                        }
                                        if sy {
                                            y *= -1;
                                        }
                                        map.entry((x, y)).or_insert(1);
                                    }
                                }
                            }
                        }

                        self.visibility.push(map.len() as i32 - 1_i32);
                    }
                }
            }
        }
    }

    pub fn get_starting_position(&mut self) -> Option<(isize, isize)> {
        match self.starting_position {
            None => {
                self.compute_visibility();
                let max = *self.visibility.iter().max()?;
                let index = self.visibility.iter().position(|&x| x == max)?;
                self.starting_position = Some((
                    index as isize % self.width as isize,
                    index as isize / self.height as isize,
                ));
                self.starting_position
            }
            Some(s) => Some(s),
        }
    }

    pub fn get_maximum_visibility(&mut self) -> Option<&i32> {
        match self.starting_position {
            None => {
                self.get_starting_position();
                self.get_maximum_visibility()
            }
            Some(_) => self.visibility.iter().max(),
        }
    }

    pub fn destroy_asteroids_in_megacannon_order(&mut self) -> Vec<Asteroid> {
        let start = self.get_starting_position().unwrap();
        let mut temp_storage: Vec<Vec<Asteroid>> = Vec::new();

        // This gets us all the possible top-right 1/8th moves
        let mut farey_quadrants: Vec<_> = farey_sequence(self.width.max(self.height) as isize);
        // This gets us all the possible top-right 1/4th moves
        farey_quadrants.extend(
            farey_quadrants
                .clone()
                .into_iter()
                .map(|f| Frac {
                    numerator: f.denominator,
                    denominator: f.numerator,
                })
                .rev()
                .skip(1),
        );
        // This gets us all the possible right half moves
        farey_quadrants.extend(
            farey_quadrants
                .clone()
                .into_iter()
                .map(|f| Frac {
                    numerator: f.numerator,
                    denominator: -f.denominator,
                })
                .rev()
                .skip(1),
        );
        // This gets us all the possible moves :)
        farey_quadrants.extend(
            farey_quadrants
                .clone()
                .into_iter()
                .map(|f| Frac {
                    numerator: -f.numerator,
                    denominator: f.denominator,
                })
                .rev()
                .skip(1),
        );

        // Beginning and end overlap, pop last element
        farey_quadrants.pop();

        // Transform the quadrants into our coordinates.
        // For example, Frac { numerator: 2, denominator: 3 } means { x: 3, y: -2 }
        farey_quadrants = farey_quadrants
            .iter()
            .map(|f| Frac {
                numerator: f.numerator,
                denominator: -f.denominator,
            })
            .collect();

        for Frac {
            numerator: dx,
            denominator: dy,
        } in &farey_quadrants
        {
            let dx = *dx;
            let dy = *dy;
            let mut i = 0;
            let (mut cx, mut cy) = (start.0 + dx, start.1 + dy);

            // This loop applies (dx, dy) to (cx, cy) until it goes out of bound.
            // If an asteroid is found, it's pushed. Each (dx, dy) couple fills `x` array.
            // All the arrays are flattened in the end, ensuring that the asteroids found first have
            // the higher priority.
            while 0 <= cx && cx < self.width as isize && 0 <= cy && cy < self.height as isize {
                if let Some(a) = &self.asteroids[cy as usize * self.width as usize + cx as usize] {
                    if temp_storage.len() <= i {
                        temp_storage.push(vec![]);
                    }
                    temp_storage[i].push(*a);
                    i += 1;
                }

                cx += dx;
                cy += dy;
            }
        }

        temp_storage.into_iter().flatten().collect()
    }
}

fn solve_part1(input: &str) -> Box<dyn std::fmt::Display> {
    let mut a = AsteroidsField::from(input);
    let res = *a.get_maximum_visibility().unwrap();

    Box::new(res)
}

fn solve_part2(input: &str) -> Box<dyn std::fmt::Display> {
    let mut a = AsteroidsField::from(input);
    let res = a
        .destroy_asteroids_in_megacannon_order()
        .get(199)
        .map(|a| a.x * 100 + a.y)
        .unwrap();

    Box::new(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut a = AsteroidsField::from(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        );
        assert_eq!(33, *a.get_maximum_visibility().unwrap());

        let mut a = AsteroidsField::from(
            "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
        );
        assert_eq!(35, *a.get_maximum_visibility().unwrap());

        let mut a = AsteroidsField::from(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );
        assert_eq!(210, *a.get_maximum_visibility().unwrap());

        let mut a = AsteroidsField::from(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );
        assert_eq!(Some((11, 13)), a.get_starting_position());
        assert_eq!(
            802,
            a.destroy_asteroids_in_megacannon_order()
                .get(199)
                .map(|a| a.x * 100 + a.y)
                .unwrap()
        );
    }
}
