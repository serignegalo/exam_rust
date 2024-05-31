#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);
impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut res = Vec::new();
        for col in slice.iter() {
            let mut temp = Vec::new();
            for row in col.iter() {
                temp.push(*row);
            }
            res.push(temp);
        }
        Matrix(res)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, col) in self.0.iter().enumerate() {
            write!(f, "(")?;
            for (j, row) in col.iter().enumerate() {
                if j == 0 {
                    write!(f, "{}", row)?;
                } else {
                    write!(f, " {}", row)?;
                }
            }
            if i < self.0.len() - 1 {
                write!(f, ")\n")?;
            } else {
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
