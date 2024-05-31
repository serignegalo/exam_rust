pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {return "".to_string();}
    input[..1].to_ascii_uppercase() + &input[1..].to_ascii_lowercase()
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    for word in input.split_whitespace() {
        res.push_str(&capitalize_first(word));
        res.push(' ');
    }
    let res = res.trim();
    res.to_string()
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c.is_ascii_lowercase() {
            res.push(c.to_ascii_uppercase());
        } else if  c.is_ascii_uppercase() {
            res.push(c.to_ascii_lowercase());
        } else {
            res.push(c);
        }
    }
    res
} 
