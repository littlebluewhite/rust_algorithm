pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let n = grid.len();
    let m = grid[0].len();
    let total = n * m;
    let shift = k as usize % total;
    let mut res: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            let idx = (i * m + j + shift) % total;
            let new_i = idx / m;
            let new_j = idx % m;
            res[new_i][new_j] = grid[i][j];
        }
    }
    res
}