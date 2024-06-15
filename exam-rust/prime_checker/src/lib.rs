#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 {
        return None;
    }
    if nb == 2 {
        return Some(Ok(nb));
    }

    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    }
    let max = (nb as f64).sqrt() as u32;
    for i in (3..=max).step_by(2) {
        if nb % i == 0 {
            return Some(Err(PrimeErr::Divider(i)));
        }
    }
    return Some(Ok(nb));
}
