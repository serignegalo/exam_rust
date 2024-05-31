use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	IV,
	V,
	IX,
	X,
	XL,
	L,
	XC,
	C,
	CD,
	D,
	CM,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        let tab = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let roman = [RomanDigit::M, RomanDigit::CM, RomanDigit::D,RomanDigit::CD, RomanDigit::C, RomanDigit::XC, RomanDigit::L, RomanDigit::XL, RomanDigit::X, RomanDigit::IX, RomanDigit::V, RomanDigit::IV, RomanDigit::I];
        let mut res = Vec::new();
		if n == 0 {
			res.push(RomanDigit::Nulla);
		}let mut res = Vec::new();
		if n == 0 {
			res.push(RomanDigit::Nulla);
		}
        for i in 0..tab.len() {
			while n >= tab[i] {
				if roman[i]==RomanDigit::CM{
				res.push(RomanDigit::C);
				res.push(RomanDigit::M);
				} else if roman[i]==RomanDigit::CD{
					res.push(RomanDigit::C);
					res.push(RomanDigit::D);
				}  else if roman[i]==RomanDigit::XC{
					res.push(RomanDigit::X);
					res.push(RomanDigit::C);
				}  else if roman[i]==RomanDigit::XL{
					res.push(RomanDigit::X);
					res.push(RomanDigit::L);
				}  else if roman[i]==RomanDigit::IX{
					res.push(RomanDigit::I);
					res.push(RomanDigit::X);
				}  else if roman[i]==RomanDigit::IV{
					res.push(RomanDigit::I);
					res.push(RomanDigit::V);
				} else {
					res.push(roman[i]);
				}
				n -= tab[i];
			}
		}
        RomanNumber(res)
    }
}
