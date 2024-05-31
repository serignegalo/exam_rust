pub fn reverse_it(v: i32) -> String {
    let mut res = String::new();
    let mut new_v = v;
    if v < 0 {
        res.push_str("-");
        new_v = new_v * -1;
    }
    for i in (new_v.to_string().chars()).rev() {
        res.push(i);
    }
    res.push_str(&new_v.to_string());
    res
}
