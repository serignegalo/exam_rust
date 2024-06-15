/*
#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut a = Vec::new();
        for row in slice {
            a.push(row.to_vec());
        };

        Matrix(a)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, "\n")?;
            }
            write!(f, "(")?;
            for (j, &value) in row.iter().enumerate() {
                if j != 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", value)?;
            }
            write!(f, ")")?;
        };
        Ok(())
    }
}

*/

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut result: Vec<Vec<i32>> = Vec::new();
        slice.iter().for_each(|&row| result.push(Vec::from(row)));
        Matrix(result)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self
            .0
            .iter()
            .map(|row| {
                format!(
                    "({})",
                    row.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", a)
    }
}
fn main() {
    let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    println!("{}", matrix);
}
