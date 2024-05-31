pub fn rot21(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c as u8 >= b'a' && c as u8 <= b'e' || c as u8 >= b'A' && c as u8 <= b'E' {
            let temp = (c as u8 + 21) as char;
            res.push(temp);
        } else if c as u8 >= b'f' && c as u8 <= b'z' || c as u8 >= b'F' && c as u8 <= b'Z' {
            let temp = (c as u8 - 5) as char;
            res.push(temp);
        } else {
            res.push(c);
        }
    }
    res
}
