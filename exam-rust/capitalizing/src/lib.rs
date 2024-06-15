pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    input[0..1].to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    input.split_whitespace().map(|word| {
        capitalize_first(word)
    }).collect::<Vec<String>>().join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_uppercase() {
            c.to_lowercase().next().unwrap()
        } else {
            c.to_uppercase().next().unwrap()
        }
    }).collect()
}
