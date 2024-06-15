use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);
impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> RomanNumber {
        use RomanDigit::*;
        
        if value == 0 {
            return RomanNumber(vec!(Nulla))
        }

        let mut roman = Vec::new();
        while value > 0 {
            roman.extend( match value {
                    _ if value >= 1000 => {value -= 1000; vec![M]},
                    _ if value >= 900 => {value -= 900; vec![C, M]},
                    _ if value >= 500 => {value -= 500; vec![D]},
                    _ if value >= 400 => {value -= 400; vec![C, D]},
                    _ if value >= 100 => {value -= 100; vec![C]},
                    _ if value >= 90 => {value -= 90; vec![X, C]},
                    _ if value >= 50 => {value -= 50; vec![L]},
                    _ if value >= 40 => {value -= 40; vec![X, L]},
                    _ if value >= 10 => {value -= 10; vec![X]},
                    _ if value >= 9 => {value -= 9; vec![I, X]},
                    _ if value >= 5 => {value -= 5; vec![V]},
                    _ if value >= 4 => {value -= 4; vec![I, V]},
                    _ =>  { value -= 1; vec![I]}
                } 
            );
        }

        RomanNumber(roman)
    }
}
