pub fn is_name_char(c: char) -> bool {
    is_name_first_char(c) || c.is_ascii_digit()
}

pub fn is_name_first_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
