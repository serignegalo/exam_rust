use std::collections::HashMap;

/*pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values().cloned().max().unwrap_or_default()
}
 */
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    *h.values().max().unwrap_or(&0)
}
