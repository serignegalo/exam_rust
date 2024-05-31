
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError{validation, expected}
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }
    let mut res = String::new();
    for c in original.chars() {
        if c.is_ascii_lowercase() {
            let temp = (b'z' - c as u8 + b'a') as char;
            res.push_str(&temp.to_string());
        } else if c.is_ascii_uppercase() {
            let temp = (b'Z' - c as u8 + b'A') as char;
            res.push_str(&temp.to_string());
        } else {
            res.push(c);
        }
    }
    if res == ciphered {
        return Some(Ok(true));
    } else {
        return Some(Err(CipherError::new(false, res)));
    }
}
