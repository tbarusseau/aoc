#![allow(dead_code)]

use itertools::Itertools;

pub fn get_all_neighbouring_pos(
    pos: (usize, usize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    const OFFSETS: &[(isize, isize)] = &[
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let (px, py) = pos;

    OFFSETS
        .iter()
        .map(|offset| {
            let (dx, dy) = offset;

            let x = (px as isize + dx).clamp(0, width as isize - 1) as usize;
            let y = (py as isize + dy).clamp(0, height as isize - 1) as usize;

            (x, y)
        })
        .dedup()
        .collect_vec()
}

pub fn get_orthogonal_neighbouring_pos(
    pos: (usize, usize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    const OFFSETS: &[(isize, isize)] = &[(0, 1), (-1, 0), (1, 0), (0, -1)];

    let (px, py) = pos;

    OFFSETS
        .iter()
        .map(|offset| {
            let (dx, dy) = offset;

            let x = (px as isize + dx).clamp(0, width as isize - 1) as usize;
            let y = (py as isize + dy).clamp(0, height as isize - 1) as usize;

            (x, y)
        })
        .dedup()
        .collect_vec()
}
