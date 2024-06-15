#[derive(Debug, Clone)]
pub struct Garage {
    left: Option<i32>,
    right: Option<i32>,
}


impl Garage {
    pub fn new(left: Option<i32>, right: Option<i32>) -> Self {
        Garage { left, right }
    }

    pub fn move_to_right(&mut self) {
        if let Some(val) = self.left {
            self.right = Some(val);
            self.left = None;
        }
    }

    pub fn move_to_left(&mut self) {
        if let Some(val) = self.right {
            self.left = Some(val);
            self.right = None;
        }
    }
}

fn main() {
    let mut garage = Garage {
        left: Some(5),
        right: Some(2),
    };

    println!("{:?}", garage);
    garage.move_to_right();
    println!("{:?}", garage);
    garage.move_to_left();
    println!("{:?}", garage);
}

// And its output

// $ cargo run
// Garage { left: Some(5), right: Some(2) }
// Garage { left: None, right: Some(7) }
// Garage { left: Some(7), right: None }
// $