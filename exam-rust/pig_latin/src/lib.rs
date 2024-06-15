pub fn pig_latin(text: &str) -> String {
    println!("------{text}");
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut pig_str = text.to_string();
    let first_char = text.chars().next().unwrap();
    if vowels.contains(&first_char) {
        pig_str.push_str("ay");
        return pig_str;
    } else {
        if text.chars().nth(1).unwrap() == 'q' && text.chars().nth(2).unwrap() == 'u' {
            pig_str = pig_str.chars().skip(3).collect();
            pig_str.push(first_char);
            pig_str.push_str("qu");
            return pig_latin(&pig_str.clone());
        }
        pig_str = pig_str.chars().skip(1).collect();
        pig_str.push(first_char);
        return pig_latin(&pig_str.clone());
    }
}
