pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let row = mat.len();
    let column = mat[0].len();
    let mut max_side = row.min(column);
    let mut prefix = vec![vec![0i64; column + 1]; row + 1];
    for r in 0..row {
        for c in 0..column {
            prefix[r + 1][c + 1] =
                prefix[r + 1][c] + prefix[r][c + 1] - prefix[r][c] + mat[r][c] as i64;
        }
    }
    while max_side > 0 {
        for i in 0..=row - max_side {
            for j in 0..=column - max_side {
                let sum = prefix[i + max_side][j + max_side]
                    - prefix[i + max_side][j]
                    - prefix[i][j + max_side]
                    + prefix[i][j];
                if sum <= threshold as i64 {
                    return max_side as i32;
                }
            }
        }
        max_side -= 1;
    }
    max_side as i32
}
