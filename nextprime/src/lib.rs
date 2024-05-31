pub fn next_prime(nbr: u64) -> u64 {
    if !is_prime(nbr) {
        return next_prime(nbr + 1);
    }
    nbr
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
