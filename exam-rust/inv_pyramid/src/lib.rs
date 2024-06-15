pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    if i == 0 {
        return Vec::new();
    }

    let capacity = 2 * i as usize - 1;
    let mut output = vec![String::new(); capacity];


    for j in 0..i {
        let str = format!("{}{}", " ".repeat(j as usize + 1), v.repeat(j as usize + 1));
        output[j as usize] = str.clone();
        if j != i - 1 {
            output[capacity - j as usize - 1] = str.clone();
        }
    };
    output
}