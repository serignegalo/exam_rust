pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    let mut res = String::new();
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }
    let num_lines = (s.len() as f32 / letters_per_turn as f32).ceil();
    let str_val = format!(
        "{}{}",
        s,
        " ".repeat(num_lines as usize * letters_per_turn as usize - s.len())
    );
    let chars: Vec<char> = str_val.chars().collect();

    for i in 0..letters_per_turn as usize {
        res.push(chars[i]);
        let mut j = i.clone();
        while j + (letters_per_turn as usize) < str_val.len() {
            j += letters_per_turn as usize;
            res.push(chars[j]);
        }
    }

    Some(res.trim().to_string())
}
