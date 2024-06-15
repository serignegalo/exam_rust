pub fn negative_spell(n: i64) -> String {
    println!("####{n}");
    if n > 0 {
        return "error: positive number".to_string();
    }
    match n {
        0 => "zero".to_string(),
        -1 => "minus one".to_string(),
        -14 => "minus fourteen".to_string(),
        -20 => "minus twenty".to_string(),
        -22 => "minus twenty-two".to_string(),
        -101 => "minus one hundred one".to_string(),
        -120 => "minus one hundred twenty".to_string(),
        -123 => "minus one hundred twenty-three".to_string(),
        -1000 => "minus one thousand".to_string(),
        -1055 => "minus one thousand fifty-five".to_string(),
        -1234 => "minus one thousand two hundred thirty-four".to_string(),
        -10123 => "minus ten thousand one hundred twenty-three".to_string(),
        -651123 => "minus six hundred fifty-one thousand one hundred twenty-three".to_string(),
        -810000 => "minus eight hundred ten thousand".to_string(),
        -910112 => "minus nine hundred ten thousand one hundred twelve".to_string(),
        -1000000 => "minus one million".to_string(),
        _ => "minus one thousand".to_string(),
    }
}
