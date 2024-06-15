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
        match (&self.antigen, &self.rh_factor) {
            (Antigen::A, RhFactor::Positive) => match (&other.antigen, &other.rh_factor) {
                (Antigen::A, _) | (Antigen::O, _) => true,
                _ => false,
            },
            (Antigen::A, RhFactor::Negative) => match (&other.antigen, &other.rh_factor) {
                (Antigen::A, RhFactor::Negative) | (Antigen::O, RhFactor::Negative) => true,
                _ => false,
            },
            (Antigen::O, RhFactor::Positive) => match (&other.antigen, &other.rh_factor) {
                (Antigen::O, _) => true,
                _ => false,
            },
            (Antigen::O, RhFactor::Negative) => match (&other.antigen, &other.rh_factor) {
                (Antigen::O, RhFactor::Negative) => true,
                _ => false,
            },
            (Antigen::B, RhFactor::Positive) => match (&other.antigen, &other.rh_factor) {
                (Antigen::B, _) | (Antigen::O, _) => true,
                _ => false,
            },
            (Antigen::B, RhFactor::Negative) => match (&other.antigen, &other.rh_factor) {
                (Antigen::B, RhFactor::Negative) | (Antigen::O, RhFactor::Negative) => true,
                _ => false,
            },
            (Antigen::AB, RhFactor::Positive) => true,
            (Antigen::AB, RhFactor::Negative) => match (&other.antigen, &other.rh_factor) {
                (Antigen::A, RhFactor::Negative) | (Antigen::B, RhFactor::Negative) |
                (Antigen::AB, RhFactor::Negative) | (Antigen::O, RhFactor::Negative) => true,
                _ => false,
            },
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_bloods = vec![
            Self{ antigen:Antigen::A, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::A, rh_factor: RhFactor::Negative},
            Self{ antigen:Antigen::B, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::B, rh_factor: RhFactor::Negative},
            Self{ antigen:Antigen::O, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::O, rh_factor: RhFactor::Negative},
            Self{ antigen:Antigen::AB, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::AB, rh_factor: RhFactor::Negative},
        ];
        all_bloods.into_iter().filter(|bt| self.can_receive_from(bt)).collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
        
        let all_bloods = vec![
            Self{ antigen:Antigen::A, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::A, rh_factor: RhFactor::Negative},
            Self{ antigen:Antigen::B, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::B, rh_factor: RhFactor::Negative},
            Self{ antigen:Antigen::O, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::O, rh_factor: RhFactor::Negative},
            Self{ antigen:Antigen::AB, rh_factor: RhFactor::Positive},
            Self{ antigen:Antigen::AB, rh_factor: RhFactor::Negative},
        ];
        all_bloods.into_iter().filter(|bt| bt.can_receive_from(self)).collect()
    }
}
