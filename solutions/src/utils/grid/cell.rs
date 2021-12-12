use grid::Grid;

pub struct Cell<'a, T> {
    pub value: &'a T,
    pub flat_index: usize,
    pub index: (usize, usize),
    grid: &'a Grid<T>,
} 

impl<'a, T> Cell<'a, T> {
    pub fn new(grid: &'a Grid<T>, flat_index: usize, value: &'a T) -> Cell<'a, T> {
        Cell {
            value,
            flat_index,
            index: (flat_index % grid.cols(), flat_index / grid.cols()),
            grid,
        }
    }
}