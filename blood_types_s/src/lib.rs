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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.antigen == Antigen::A || self.antigen == Antigen::B || self.antigen == Antigen::O { 
            if self.rh_factor == RhFactor::Positive {
                if other.antigen == self.antigen || other.antigen == Antigen::O {
                    return true;
                };
            } else {
                if (other.antigen == self.antigen || other.antigen == Antigen::O) && other.rh_factor == RhFactor::Negative {
                    return true;
                };
            };
        } else if self.antigen == Antigen::AB {
            if self.rh_factor == RhFactor::Positive {
                return true;
            } else if self.rh_factor == RhFactor::Negative {
                if (other.antigen == self.antigen || other.antigen == Antigen::A || other.antigen == Antigen::B || other.antigen == Antigen::O) && other.rh_factor == RhFactor::Negative {
                    return true;
                };
            };
        };
        return false;
	}

	pub fn donors(&self) -> Vec<Self> {
        let mut res = Vec::new();
        for antigen in [Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for rh_factor in [RhFactor::Positive, RhFactor::Negative] {
                let lolo = BloodType{antigen: antigen.clone(), rh_factor: rh_factor.clone()};
                if self.can_receive_from(&lolo) {
                    res.push(lolo);
                }
            }
        }
        res
	}

	pub fn recipients(&self) -> Vec<Self> {
        let mut res = Vec::new();
        for antigen in [Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for rh_factor in [RhFactor::Positive, RhFactor::Negative] {
                let lolo = BloodType{antigen: antigen.clone(), rh_factor: rh_factor.clone()};
                if lolo.can_receive_from(self) {
                    res.push(lolo);
                }
            }
        }
        res
	}
}
