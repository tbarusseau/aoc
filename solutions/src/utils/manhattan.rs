pub fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    (b.0 - a.0).unsigned_abs() + (b.1 - a.1).unsigned_abs()
}
