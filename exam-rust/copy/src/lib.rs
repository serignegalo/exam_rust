pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value = (c as f64).exp();
    let log_natural_value = (c as f64).abs().ln();
    (c, exp_value, log_natural_value)
}

pub fn str_function(a: String) -> (String, String) {
    let mut exp_values: Vec<String> = Vec::new();

    for value in a.split_whitespace() {
        let exp_value: f64 = value.parse().unwrap_or(0.0);
        exp_values.push(exp_value.exp().to_string());
    }

    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut log_values: Vec<f64> = Vec::new();

    for value in b.iter() {
        log_values.push((*value as f64).abs().ln());
    }

    (b, log_values)
}
