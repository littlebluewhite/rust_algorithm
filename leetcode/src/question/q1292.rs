pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let row = mat.len();
    let column = mat[0].len();
    let mut max_side = row.min(column);
    let mut prefix = vec![vec![0; column+1]; row];
    for r in 0..row {
        for c in 0..column {
            prefix[r][c+1] = prefix[r][c] + mat[r][c];
        }
    }
    while max_side > 0 {
        for i in 0..=row-max_side {
            'a: for j in 0..=column-max_side {
                let mut sum = 0;
                for r in i..i+max_side {
                    sum += prefix[r][j+max_side] - prefix[r][j];
                    if sum > threshold {
                        continue 'a;
                    }
                }
                return max_side as i32;
            }
        }
        max_side -= 1;
    }
    max_side as i32
}