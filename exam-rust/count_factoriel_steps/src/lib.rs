pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut counter: u64 = 0;
    let mut result: u64 = 1;
    let mut next_value: u64 = 1;

    while result < factorial {
        counter += 1;
        result *= next_value;
        next_value += 1;
    }
    if result == factorial {
        return counter;
    }
    0
}

// fn main() {
//     println!(
//         "The factorial steps of 720 = {}",
//         count_factorial_steps(720)
//     );
//     println!("The factorial steps of 13 = {}", count_factorial_steps(13));
//     println!("The factorial steps of 6 = {}", count_factorial_steps(6));
// }

// $ cargo run
// The factorial steps of 720 = 6
// The factorial steps of 13 = 0
// The factorial steps of 6 = 3
