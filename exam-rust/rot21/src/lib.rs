pub fn rot21(input: &str) -> String {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut output = String::new();

    for ch in input.chars() {
        if !letters.contains(ch.to_ascii_lowercase()) {
            output.push(ch);
            continue;
        };

        let position = letters.chars().position(|c| c == ch.to_ascii_lowercase()).unwrap_or_default();
        let corresponding_letter = letters.chars().nth((position + 21) % 26).unwrap_or_default();
        // println!("rot char : {} position: {} c_position: {} letters: {}", ch, position, (position + 21) % 26, corresponding_letter);
        
        if ch.is_ascii_uppercase() {
            output.push(corresponding_letter.to_ascii_uppercase());
        } else {
            output.push(corresponding_letter)
        }
    };

    output
}