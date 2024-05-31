pub fn prev_prime(nbr: u64) -> u64  {
    if nbr <= 1 {
        return 0;
    } 
    let n = nbr - 1;
    if !is_prime(n) {
        return prev_prime(n);
    }
    n
}

pub fn is_prime(nbr: u64) -> bool  {
    if nbr < 2 {
        return false;
    } 
    for i in 2..=nbr/2 {
        if nbr % i == 0 {
            return false;
        }
    }
    true
}
