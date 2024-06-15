pub fn reverse_it(v: i32) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let binding = v.to_string();
    let num = binding.trim_matches(|c| c == '-');
    let reverse = num.to_string().chars().rev().collect::<String>();
    format!("{}{}{}", sign, reverse, num)
}
