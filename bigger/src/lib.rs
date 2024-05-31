use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut big = 0;
    for (_k, v) in h {
        if v > big {
            big = v;
        }
    }
    big
}