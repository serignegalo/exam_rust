pub fn scytale_cipher(message: String, i: u32) -> String {
    if message.len() == 0 || i == 0 {
        return message;
    }

    let num_rows = (message.len() as f64 / i as f64).ceil() as usize;
    let mut grid = vec![vec![' '; i as usize]; num_rows];

    for (idx, c) in message.chars().enumerate() {
        let row = idx / i as usize;
        let col = idx % i as usize;
        grid[row][col] = c;
    }
    
    let mut output = String::new();

    for c in 0..i {
        for r in 0..num_rows {
            output.push(grid[r][c as usize]);
        }
    }

    output.trim().to_string()
}
/* Other Solution
pub fn scytale_cipher(message: String, wraps: u32) -> String {
    let padded_message = format!(
        "{:<width$}",
        message,
        width = (message.len() as f32 / wraps as f32).ceil() as usize * wraps as usize
    );
    (0..wraps)
        .flat_map(|j| padded_message.chars().skip(j as usize).step_by(wraps as usize))
        .collect::<String>()
        .trim()
        .to_string()
}

 */