#![allow(unused)]

use super::direction::Direction;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
}

impl<T> From<Matrix<T>> for Vec<Vec<T>> {
    fn from(val: Matrix<T>) -> Self {
        val.0
    }
}

impl<T> Matrix<T>
where
    T: Copy + std::cmp::PartialEq,
{
    pub fn rotate_right(&mut self) {
        if self.0.is_empty() || self.0.len() == 1 {
            return;
        }

        self.transpose();

        // Reverse each row
        for i in 0..self.0.len() {
            self.0[i].reverse();
        }
    }

    pub fn transpose(&mut self) {
        if self.0.is_empty() || self.0.len() == 1 {
            return;
        }

        for i in 0..self.0.len() {
            for j in i..self.0[0].len() {
                let tmp = self.0[i][j];
                self.0[i][j] = self.0[j][i];
                self.0[j][i] = tmp;
            }
        }
    }

    pub fn edge_matches(&self, other: &Self, direction: Direction) -> bool {
        let len = self.0.len();

        let [left, right] = match direction {
            Direction::Up => [self.0[0].clone(), other.0[len - 1].clone()],
            Direction::Down => [self.0[len - 1].clone(), other.0[0].clone()],
            Direction::Right => {
                let mut l = Vec::with_capacity(len);
                let mut r = Vec::with_capacity(len);

                for row in &self.0 {
                    l.push(row[len - 1]);
                }

                for row in &other.0 {
                    r.push(row[0]);
                }

                [l, r]
            }
            Direction::Left => {
                let mut l = Vec::with_capacity(len);
                let mut r = Vec::with_capacity(len);

                for row in &self.0 {
                    l.push(row[0]);
                }

                for row in &other.0 {
                    r.push(row[len - 1]);
                }

                [l, r]
            }
        };

        left == right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_2x2() {
        let mut m = Matrix::from(vec![vec![1, 2], vec![3, 4]]);

        m.rotate_right();
        assert_eq!(m, vec![vec![3, 1], vec![4, 2]].into());

        m.rotate_right();
        assert_eq!(m, vec![vec![4, 3], vec![2, 1]].into());

        m.rotate_right();
        assert_eq!(m, vec![vec![2, 4], vec![1, 3]].into());
    }

    #[test]
    fn test_rotate_3x3() {
        let mut m = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        m.rotate_right();
        assert_eq!(m, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],].into());

        m.rotate_right();
        assert_eq!(m, vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1],].into());

        m.rotate_right();
        assert_eq!(m, vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7],].into());
    }

    #[test]
    fn test_edge_matches() {
        let a = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from(vec![vec![2, 1], vec![4, 3]]);

        assert!(a.edge_matches(&b, Direction::Right));
        assert!(b.edge_matches(&a, Direction::Left));

        let a = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from(vec![vec![3, 4], vec![1, 2]]);

        assert!(a.edge_matches(&b, Direction::Up));
        assert!(b.edge_matches(&a, Direction::Down));
    }
}
