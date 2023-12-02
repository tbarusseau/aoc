use super::iterators::neighbours_into_iterator::NeighboursIntoIterator;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Neighbours<T> {
    pub up: Option<T>,
    pub right: Option<T>,
    pub down: Option<T>,
    pub left: Option<T>,
}

impl<T> Neighbours<&T> {
    pub fn iter(&self) -> NeighboursIntoIterator<&T> {
        NeighboursIntoIterator {
            index: 0,
            neighbours: self,
        }
    }
}
