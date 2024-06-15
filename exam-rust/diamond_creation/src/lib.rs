pub fn get_diamond(c: char) -> Vec<String> {
    let size = c as u8 - b'A';
    let mut diamond = Vec::new();

    for ch in b'A'..=c as u8 {
        let spaces = (size - (ch - b'A')) as usize;
        let mut line = String::new();
        line.push_str(&" ".repeat(spaces));
        line.push(ch as char);
        if ch != b'A' {
            let row = (ch - b'A') * 2 - 1;
            line.push_str(&" ".repeat(row as usize));
            line.push(ch as char);
            // row += 2;
        }
        line.push_str(&" ".repeat(spaces));
        diamond.push(line);
    }
    for ch in (b'A'..c as u8).rev() {
        let spaces = (size - (ch - b'A')) as usize;
        let mut line = String::new();
        line.push_str(&" ".repeat(spaces));
        line.push(ch as char);

        if ch != b'A' {
            let row = (ch - b'A') * 2 - 1;
            line.push_str(&" ".repeat(row as usize));
            line.push(ch as char);
            // row -= 2;
        }
        line.push_str(&" ".repeat(spaces));
        diamond.push(line);
    }
    diamond
}
