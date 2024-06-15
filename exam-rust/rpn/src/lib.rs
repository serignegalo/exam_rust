pub fn rpn(s: &str) -> String {
    let mut stacks = Vec::new();

    for operand in s.split_whitespace() {
        match operand {
            "+" | "-" | "/" | "%" => {
                if stacks.len() < 2 { 
                    return "Error".to_string();
                }

                let a = stacks.pop().unwrap_or(0);
                let b = stacks.pop().unwrap_or(0);
                let result = match operand {
                    "+" => a+b,
                    "-" => a-b,
                    "/" => a/b,
                    "%" => a%b,
                    _ => todo!()
                };
                stacks.push(result); 
            },
            number => {
                if let Ok(num) = number.parse::<i64>() {
                    stacks.push(num);
                } else {
                    return "Error".to_string();
                }
            }
        }
    };  

    if stacks.len() == 1 {
        stacks[0].to_string()
    } else {
        "Error".to_string()
    }
}
