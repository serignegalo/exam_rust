//This function receives a number n and returns the nth number in the Lucas Numbers where the nth number is the sum of the previous two numbers in the series.
//
// The Lucas Numbers start like this: 2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, etc...
pub fn lucas_number(n: u32) -> u32 {
    return if n == 0 {
        2
    } else if n == 1 {
        1
    } else {
        let mut a = 2;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}