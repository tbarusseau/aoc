pub fn try_index_from_char(c: char) -> Option<u8> {
    match c {
        'a'..='z' => Some((c as u8) - b'a'),
        'A'..='Z' => Some((c as u8) - b'A'),
        _ => None,
    }
}

pub fn index_from_char(c: char) -> u8 {
    try_index_from_char(c).expect("invalid char")
}

pub fn try_index_to_char(index: u8, uppercase: bool) -> Option<char> {
    let offset = match uppercase {
        true => b'A',
        false => b'a',
    };

    match index {
        0..=25 => Some((index + offset) as char),
        _ => None,
    }
}

pub fn index_to_char(index: u8, uppercase: bool) -> char {
    try_index_to_char(index, uppercase).expect("invalid index")
}
