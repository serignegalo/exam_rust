fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn next_prime(nbr: u64) -> u64 {
    if nbr <= 1 {
        return 2;
    }

    let mut step = nbr;
    loop {
        if is_prime(step) {
            return step;
        }
        step += 1;
    }
}
