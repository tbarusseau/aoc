pub fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    (b.0 - a.0).unsigned_abs() + (b.1 - a.1).unsigned_abs()
}

#[cfg(test)]
mod test {
    use crate::utils::manhattan::manhattan_distance;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(5, manhattan_distance((2, 0), (0, 3)));
    }
}
