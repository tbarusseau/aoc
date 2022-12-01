use crate::utils::grid::{neighbours::Neighbours, Grid};

pub struct GridIntoNeighboursIterator<'a, T> {
    pub grid: &'a Grid<T>,
    pub index: usize,
}

impl<'a, T> Iterator for GridIntoNeighboursIterator<'a, T> {
    type Item = (&'a T, Neighbours<&'a T>);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.index % self.grid.width;
        let y = self.index / self.grid.width;

        let result = self.grid.get(x, y)?;

        let up = y.checked_sub(1).and_then(|n| self.grid.get(x, n));
        let right = self.grid.get(x + 1, y);
        let down = self.grid.get(x, y + 1);
        let left = x.checked_sub(1).and_then(|n| self.grid.get(n, y));

        self.index += 1;

        Some((
            result,
            Neighbours {
                up,
                right,
                down,
                left,
            },
        ))
    }
}
