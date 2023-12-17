#[allow(dead_code)]
pub fn get_width_height<T>(v: &[Vec<T>]) -> (usize, usize) {
    let height = v.len();
    assert!(height > 0);
    let width = v[0].len();

    (width, height)
}
