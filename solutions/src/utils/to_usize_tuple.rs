pub fn to_usize_tuple(
    input: (isize, isize),
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    let (x, y) = input;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        None
    } else {
        Some((x as usize, y as usize))
    }
}
