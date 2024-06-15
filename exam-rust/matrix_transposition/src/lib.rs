#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (u32, u32), pub (u32, u32));
pub fn transpose(m: Matrix) -> Matrix {
    Matrix((m.0.0, m.1.0), (m.0.1, m.1.1))
}
