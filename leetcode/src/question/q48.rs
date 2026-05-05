pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for r in 0..n{
        for c in r+1..n{
            let temp = matrix[r][c];
            matrix[r][c] = matrix[c][r];
            matrix[c][r] = temp;
        }
    }
    for row in matrix.iter_mut(){
        row.reverse();
    }
}