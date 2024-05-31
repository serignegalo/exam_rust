pub fn scytale_cipher(message: String, i: u32) -> String {
    let cols = i as usize;
    let rows = (message.len() as f64 / cols as f64).ceil() as usize;
    let mut res = String::new();

    for col in 0..cols {
        for row in 0..rows {
            let index = row * cols + col;
            if index < message.len() {
                res.push(message.chars().nth(index).unwrap());
            } else {
                res.push(' ');
            }
        }
    }
    res.trim().to_string()
}