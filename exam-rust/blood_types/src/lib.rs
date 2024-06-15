#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Antigen::*;
        match s {
            "A" => Ok(A),
            "AB" => Ok(AB),
            "B" => Ok(B),
            "O" => Ok(O),
            _ => Err(())
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RhFactor::*;
        match s {
            "+" => Ok(Positive),
            "-" => Ok(Negative),
            _ => Err(())
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen != other.antigen {
            self.antigen.cmp(&other.antigen)
        } else {
            self.rh_factor.cmp(&other.rh_factor)
        }
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen = Antigen::from_str(&s[..(s.len() - 1)])?;
        let rh_factor = RhFactor::from_str(&s[(s.len() - 1)..])?;
        Ok(BloodType {antigen, rh_factor})
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Antigen::*;
        use RhFactor::*;
        write!(f,
            "{}{}",
            match self.antigen {
                A => "A",
                AB => "AB",
                B => "B",
                O => "O",
            },
            match self.rh_factor {
                Positive => "+",
                Negative => "-",
            }
        )
    }
}

impl BloodType {
    fn new(antigen: Antigen, rh_factor: RhFactor) -> Self {
        Self { antigen, rh_factor }
    }

	pub fn can_receive_from(&self, other: &Self) -> bool {
        let can_receive_from_blood = self.donors();
        
        for blood in can_receive_from_blood.iter() {
            if blood.antigen == other.antigen && blood.rh_factor == other.rh_factor {
                return true;
            }
        };
        false
	}

	pub fn donors(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;
        match (self.antigen.clone(), self.rh_factor.clone()) {
            (A, Positive) => vec!(Self::new(A, Positive), Self::new(A, Negative), Self::new(O, Positive), Self::new(O, Negative)),
            (A, Negative) => vec!(Self::new(A, Negative), Self::new(O, Negative)),

            (B, Positive) => vec!(Self::new(B, Positive), Self::new(B, Negative), Self::new(O, Positive), Self::new(O, Negative)),
            (B, Negative) => vec!(Self::new(B, Negative), Self::new(O, Negative)),

            (AB, Positive) => vec!(Self::new(A, Positive), Self::new(A, Negative), Self::new(B, Positive), Self::new(AB, Positive), Self::new(AB, Negative), Self::new(B, Negative), Self::new(O, Positive), Self::new(O, Negative)),
            (AB, Negative) => vec!(Self::new(A, Negative), Self::new(B, Negative), Self::new(AB, Negative), Self::new(O, Negative)),

            (O, Positive) => vec!(Self::new(O, Positive), Self::new(O, Negative)),
            (O, Negative) => vec!(Self::new(O, Negative)),
        }
	}

	pub fn recipients(&self) -> Vec<BloodType> {
        use Antigen::*;
        use RhFactor::*;
        match (self.antigen.clone(), self.rh_factor.clone()) {
            (A , Positive)=> vec!(Self::new(A, Positive), Self::new(AB, Positive)),
            (A , Negative)=> vec!(Self::new(A, Positive), Self::new(A, Negative), Self::new(AB, Positive), Self::new(AB, Negative)),

            (B , Positive)=> vec!(Self::new(B, Positive), Self::new(AB, Positive)),
            (B , Negative)=> vec!(Self::new(B, Positive), Self::new(B, Negative), Self::new(AB, Positive), Self::new(AB, Negative)),

            (AB, Positive) => vec!(Self::new(AB, Positive)),
            (AB, Negative) => vec!(Self::new(AB, Positive), Self::new(AB, Negative)),

            (O , Positive)=> vec!(Self::new(O, Positive), Self::new(A, Positive), Self::new(B, Positive), Self::new(AB, Positive)),
            (O , Negative)=> vec!(Self::new(O, Positive), Self::new(O, Negative), Self::new(A, Positive), Self::new(A, Negative), Self::new(B, Positive), Self::new(B, Negative), Self::new(AB, Positive), Self::new(AB, Negative)),
        }
    }
}
