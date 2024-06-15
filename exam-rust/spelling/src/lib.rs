

pub fn spell(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1..=19 => small_numbers(n),
        20..=99 => tens(n),
        100..=999 => hundreds(n),
        1000..=999999 => thousands(n),
        1000000 => "one million".to_string(),
        _ => unimplemented!("The function is not designed to handle numbers beyond one million."),
    }
}

fn small_numbers(n: u64) -> String {
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen"];
    words[(n - 1) as usize].to_string()
}

fn tens(n: u64) -> String {
    let tens = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    if n < 20 {
        return small_numbers(n);
    }
    let prefix = tens[(n / 10 - 2) as usize];
    let remainder = n % 10;
    if remainder == 0 {
        prefix.to_string()
    } else {
        format!("{}-{}", prefix, small_numbers(remainder))
    }
}

fn hundreds(n: u64) -> String {
    let hundreds = n / 100;
    let remainder = n % 100;
    let prefix = format!("{} hundred", small_numbers(hundreds));
    if remainder == 0 {
        prefix
    } else {
        format!("{} {}", prefix, if remainder < 20 { small_numbers(remainder) } else { tens(remainder) })
    }
}

fn thousands(n: u64) -> String {
    let thousands = n / 1000;
    let remainder = n % 1000;
    let prefix = format!("{} thousand", spell(thousands));
    if remainder == 0 {
        prefix
    } else if remainder < 100 {
        format!("{} {}", prefix, tens(remainder))
    } else {
        format!("{} {}", prefix, hundreds(remainder))
    }
}

/*Other Solution

fn u_to_s(u: u32) -> &'static str {
    match u {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        1000 => "thousand",
        1000000 => "million",
        _ => panic!("unexpected value"),
    }
}

fn spell_util(num: u32, result: &mut String) {
    match num {
        1..=20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => result.push_str(u_to_s(num)),
        21..=99 => {
            result.push_str(u_to_s(num - num % 10));
            result.push('-');
            spell_util(num % 10, result);
        }
        100..=999 => {
            spell_util(num / 100, result);
            result.push_str(" hundred ");
            if num % 100 != 0 {
                spell_util(num % 100, result);
            }
        }
        1000..=999_999 => {
            spell_util(num / 1000, result);
            result.push_str(" thousand ");
            if num % 1000 != 0 {
                spell_util(num % 1000, result);
            }
        }
        _ => (),
    }
}

pub fn spell(num: u32) -> String {
    if num == 0 {
        return String::from("zero");
    }
    if num == 1_000_000 {
        return String::from("one million");
    }

    let mut result = String::new();
    spell_util(num, &mut result);

    result.trim().to_string()
}
*/