pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res = Vec::new();
    for index in 1..=i {
        res.push(format!("{}{}"," ".repeat(index as usize), v.repeat(index as usize)))
    }
    for index in (1..i).rev() {
        res.push(format!("{}{}"," ".repeat(index as usize), v.repeat(index as usize)))
    }
    res
}
