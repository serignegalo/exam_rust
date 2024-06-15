pub fn lucas_number(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 1,
        _ => {
            return lucas_number(n - 1) + lucas_number(n - 2);
        }
    }
}
