#![allow(unused)]

pub mod iterators;
pub mod neighbours;
pub mod cell;

use std::{
    fmt::Display,
    iter::StepBy,
    slice::{Iter, IterMut},
};

use self::{
    iterators::grid_into_neighbours_iterator::GridIntoNeighboursIterator, neighbours::Neighbours, cell::Cell,
};

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn clear(&mut self) {
        self.data = vec![T::default(); self.width * self.height];
    }
}

impl<T> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid {
            data: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn init(width: usize, height: usize, initial_value: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid {
            data: vec![initial_value; width * height],
            width,
            height,
        }
    }

    pub fn from_data(width: usize, data: Vec<T>) -> Grid<T> {
        let l = data.len();

        if l % width != 0 {
            panic!(
                "provided data isn't a multiple of width. Expected width of {:?}",
                width
            );
        }

        let height = l / width;

        Grid {
            data,
            width,
            height,
        }
    }

    pub fn set_row(&mut self, index: usize, data: Vec<T>) {
        let range_start = index * self.width;
        let range_end = range_start + self.width;

        self.data.splice(range_start..range_end, data);
    }

    pub fn push_row(&mut self, data: Vec<T>) {
        if self.width != 0 && data.len() != self.width {
            panic!(
                "wrong row length. Expected width of {:?} but got {:?} instead.",
                self.width,
                data.len()
            );
        }

        self.width = data.len();
        self.height += 1;

        self.data.extend(data);
    }

    pub fn rows(&self) -> usize {
        self.height
    }

    pub fn cols(&self) -> usize {
        self.width
    }

    pub fn get_flat(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // pub fn get_flat(&self, index: usize) -> Option<Cell<T>> {
    //     self.data.get(index).map(|v| Cell::new(self, index, v))
    // }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data.get(y * self.width + x)
    }

    pub fn get_ortho_neighbours(&self, x: usize, y: usize) -> Option<Neighbours<&T>> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let up = y.checked_sub(1).map(|n| self.get(x, n)).flatten();
        let right = self.get(x + 1, y);
        let down = self.get(x, y + 1);
        let left = x.checked_sub(1).map(|n| self.get(n, y)).flatten();

        Some(Neighbours {
            up,
            right,
            down,
            left,
        })
    }

    pub fn get_with_ortho_neighbours(&self, x: usize, y: usize) -> Option<(&T, Neighbours<&T>)> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data
            .get(y * self.width + x)
            .map(|n| (n, self.get_ortho_neighbours(x, y).unwrap()))
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn iter_row(&self, index: usize) -> Iter<T> {
        if index >= self.height {
            panic!(
                "out of bounds. Row index must be less than {:?}, but is {:?}.",
                self.height, index
            );
        }

        let start = index * self.width;
        self.data[start..(start + self.width)].iter()
    }

    pub fn iter_col(&self, index: usize) -> StepBy<Iter<T>> {
        if index >= self.width {
            panic!(
                "out of bounds. Column index must be less than {:?}, but is {:?}.",
                self.width, index
            );
        }

        let start = index;
        self.data[start..(start * self.height)]
            .iter()
            .step_by(self.height)
    }

    pub fn iter_neighbours_ortho(&self) -> GridIntoNeighboursIterator<T> {
        GridIntoNeighboursIterator {
            grid: self,
            index: 0,
        }
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        let mut longest_elem = 0;
        for i in 0..self.rows() {
            for r in self.iter_row(i) {
                let l = r.to_string().len();
                if l > longest_elem {
                    longest_elem = l;
                }
            }
        }

        let mut line_len = 0;

        for i in 0..self.rows() {
            let mut first_passed = false;

            for r in self.iter_row(i) {
                s.push_str(&format!("| {:leftpad$} ", *r, leftpad = longest_elem));
            }
            s.push('|');

            if line_len == 0 {
                line_len = s.len();
            }

            s.push('\n');

            s.push_str(&"-".repeat(line_len));
            s.push('\n');
        }

        write!(f, "{}\n{}", "-".repeat(line_len), s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_newly_set_data() {
        let g = Grid::init(2, 2, 10);

        assert_eq!(Some(&10), g.get(0, 1));
    }

    #[test]
    fn get_neighbours_from_data() {
        let g = Grid::from_data(2, vec![0, 1, 2, 3]);

        assert_eq!(Some(&0), g.get(0, 0));
        assert_eq!(Some(&1), g.get(1, 0));
        assert_eq!(Some(&2), g.get(0, 1));
        assert_eq!(Some(&3), g.get(1, 1));

        let mut neighbours_iter = g.iter_neighbours_ortho();
        assert_eq!(
            Some((
                &0,
                Neighbours {
                    up: None,
                    right: Some(&1),
                    down: Some(&2),
                    left: None,
                }
            )),
            neighbours_iter.next()
        );
        assert_eq!(
            Some((
                &1,
                Neighbours {
                    up: None,
                    right: None,
                    down: Some(&3),
                    left: Some(&0),
                }
            )),
            neighbours_iter.next()
        );
        assert_eq!(
            Some((
                &2,
                Neighbours {
                    up: Some(&0),
                    right: Some(&3),
                    down: None,
                    left: None,
                }
            )),
            neighbours_iter.next()
        );
        assert_eq!(
            Some((
                &3,
                Neighbours {
                    up: Some(&1),
                    right: None,
                    down: None,
                    left: Some(&2),
                }
            )),
            neighbours_iter.next()
        );
        assert_eq!(None, neighbours_iter.next());
    }
}
