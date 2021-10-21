pub mod commons {
    pub fn get_nth_digit(input: u32, index: usize, left_to_right: bool) -> Option<u32> {
        let mut r = input;
        let input_len = input.to_string().len();
        if index > input_len {
            return None;
        }

        if left_to_right {
            if index == 0 {
                while r >= 10 {
                    r /= 10;
                }
                return Some(r);
            }

            for _ in 0..(input_len - index - 1) {
                r /= 10;
            }
        } else {
            for _ in 0..index {
                r /= 10;
            }
        }

        Some(r % 10)
    }
}

#[cfg(test)]
mod test_commons {
    use super::commons::*;

    #[test]
    fn test_nth_digit() {
        // Right to left
        assert_eq!(1, get_nth_digit(54321, 0, false).unwrap());
        assert_eq!(2, get_nth_digit(54321, 1, false).unwrap());
        assert_eq!(3, get_nth_digit(54321, 2, false).unwrap());
        assert_eq!(4, get_nth_digit(54321, 3, false).unwrap());
        assert_eq!(5, get_nth_digit(54321, 4, false).unwrap());

        // Left to right
        assert_eq!(5, get_nth_digit(54321, 0, true).unwrap());
        assert_eq!(4, get_nth_digit(54321, 1, true).unwrap());
        assert_eq!(3, get_nth_digit(54321, 2, true).unwrap());
        assert_eq!(2, get_nth_digit(54321, 3, true).unwrap());
        assert_eq!(1, get_nth_digit(54321, 4, true).unwrap());
    }
}
