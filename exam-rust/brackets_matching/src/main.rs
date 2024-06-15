use std::env;
use brackets_matching::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    for arg in args {
        if is_brackets_matched(&arg) {
            println!("OK");
        } else {
            println!("Error");
        }
    }
}