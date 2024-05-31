fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

pub fn rpn(s: &str) {
    if s.is_empty() {
        println!("Error");
        return;
    }
    let mut res = Vec::new();
    for v in s.split_whitespace() {
        if let Ok(n) = v.parse::<i64>() {
            res.push(n);
        } else {
            if res.len() >= 2 {
                let second = res.pop().unwrap();
                let first = res.pop().unwrap();
                match v {
                    "+" => res.push(first + second),
                    "-" => res.push(first - second),
                    "*" => res.push(first * second),
                    "/" => res.push(first / second),
                    "%" => res.push(first % second),
                    _  => {println!("Error"); return;}
                };
            } else {
                println!("Error"); 
                return;
            }
        }
    }
    if res.len() > 1 {
        println!("Error"); 
        return;
    }
    println!("{}", res[0]);
}