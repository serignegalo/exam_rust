pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let tl1 = matrix[0][0] * matrix[1][1] * matrix[2][2];
    let tl2 = matrix[0][1] * matrix[1][2] * matrix[2][0];
    let tl3 = matrix[0][2] * matrix[1][0] * matrix[2][1];

    let tr1 = matrix[0][2] * matrix[1][1] * matrix[2][0];
    let tr2 = matrix[0][1] * matrix[1][0] * matrix[2][2];
    let tr3 = matrix[0][0] * matrix[1][2] * matrix[2][1];
    
    tl1 + tl2 + tl3 - (tr1 + tr2 + tr3)
}

