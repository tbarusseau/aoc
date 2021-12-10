use crate::utils::grid::neighbours::Neighbours;

pub struct NeighboursIntoIterator<'a, T> {
    pub neighbours: &'a Neighbours<T>,
    pub index: usize,
}

impl<'a, T> Iterator for NeighboursIntoIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 4 {
            match self.index {
                0 => {
                    if self.neighbours.up.is_some() {
                        self.index += 1;
                        return self.neighbours.up.as_ref();
                    }
                }
                1 => {
                    if self.neighbours.right.is_some() {
                        self.index += 1;
                        return self.neighbours.right.as_ref();
                    }
                }
                2 => {
                    if self.neighbours.down.is_some() {
                        self.index += 1;
                        return self.neighbours.down.as_ref();
                    }
                }
                3 => {
                    if self.neighbours.left.is_some() {
                        self.index += 1;
                        return self.neighbours.left.as_ref();
                    }
                }
                _ => {
                    return None;
                }
            }

            self.index += 1;
        }

        None
    }
}
